use medtrack_application::{
    Clock, IntakeEvent, PatientStore, RegisterSchedule, SchedulerTick, SubmitIntake,
};
use medtrack_domain::{
    Duration, Instant, MedicationName, PatientId, Schedule, ScheduleConfig, ScheduleIdentity,
};
use medtrack_infrastructure::{InMemoryPatientStore, IntakeBus, StdoutReminderSink, SystemClock};
use std::sync::Arc;
use tokio::time::{Duration as TokioDuration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let store: Arc<dyn PatientStore> = Arc::new(InMemoryPatientStore::new());
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let sink = Arc::new(StdoutReminderSink);
    let register = RegisterSchedule::new(store.clone());
    let first_dose_at = clock.now().await;
    let medication = MedicationName::try_new("demo-medication")?;
    let patient = PatientId::try_new("demo-patient")?;
    let schedule = demo_schedule(patient.clone(), medication.clone(), first_dose_at)?;
    register.execute(schedule).await?;
    let reminder_patient = PatientId::try_new("reminder-patient")?;
    let reminder_medication = MedicationName::try_new("reminder-medication")?;
    let reminder_schedule = demo_schedule(reminder_patient, reminder_medication, first_dose_at)?;
    register.execute(reminder_schedule).await?;

    let submit = Arc::new(SubmitIntake::new(store.clone()));
    let (bus, bus_task) = IntakeBus::start(submit, 16);
    let bus = Arc::new(bus);
    let producer_tasks = spawn_producers(bus.clone(), patient, medication, first_dose_at);
    let scheduler = Arc::new(SchedulerTick::new(store, clock, sink));
    let scheduler_task = tokio::spawn(run_scheduler(scheduler));

    for task in producer_tasks {
        task.await??;
    }
    scheduler_task.await??;
    let bus = Arc::try_unwrap(bus).map_err(|_| "producer bus still in use")?;
    drop(bus);
    bus_task.await??;
    println!("demo complete");
    Ok(())
}

fn demo_schedule(
    patient: PatientId,
    medication: MedicationName,
    first_dose_at: Instant,
) -> Result<Schedule, medtrack_domain::DomainError> {
    let identity = ScheduleIdentity::new(patient, medication);
    let configuration = ScheduleConfig::new(
        first_dose_at,
        Duration::from_millis(250),
        Duration::from_millis(50),
        4,
    )?;
    Schedule::new(identity, configuration)
}

fn spawn_producers(
    bus: Arc<IntakeBus>,
    patient: PatientId,
    medication: MedicationName,
    first_dose_at: Instant,
) -> Vec<tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>> {
    (0..4)
        .map(|dose_index| {
            let bus = bus.clone();
            let patient = patient.clone();
            let medication = medication.clone();
            tokio::spawn(async move {
                let offset = 10 + dose_index * 250;
                let at = first_dose_at
                    .checked_add(Duration::from_millis(offset))
                    .ok_or_else(|| "demo schedule overflow".to_string())?;
                bus.send(IntakeEvent::new(patient, medication, at))
                    .await
                    .map_err(|error| Box::new(error) as Box<dyn std::error::Error + Send + Sync>)
            })
        })
        .collect()
}

async fn run_scheduler(
    scheduler: Arc<SchedulerTick>,
) -> Result<(), medtrack_application::ApplicationError> {
    let mut ticks = interval(TokioDuration::from_millis(100));
    for _ in 0..5 {
        ticks.tick().await;
        let sent = scheduler.execute().await?;
        println!("scheduler tick reminders_sent={sent}");
    }
    Ok(())
}
