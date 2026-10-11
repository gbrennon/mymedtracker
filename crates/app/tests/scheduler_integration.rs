mod fake_clock;
mod fake_sink;

use fake_clock::FakeClock;
use fake_sink::FakeReminderSink;
use medtrack_application::{
    IntakeEvent, PatientStore, RegisterSchedule, SchedulerTick, SubmitIntake,
};
use medtrack_domain::{
    Duration, Instant, MedicationName, PatientId, Schedule, ScheduleConfig, ScheduleIdentity,
};
use medtrack_infrastructure::InMemoryPatientStore;
use std::sync::Arc;

fn schedule_for(
    patient_name: &str,
    medication_name: &str,
    first_dose_at: i64,
    dose_count: usize,
) -> (ScheduleIdentity, Schedule) {
    let identity = ScheduleIdentity::new(
        PatientId::try_new(patient_name).expect("patient"),
        MedicationName::try_new(medication_name).expect("medication"),
    );
    let configuration = ScheduleConfig::new(
        Instant::from_millis(first_dose_at),
        Duration::from_millis(100),
        Duration::from_millis(10),
        dose_count,
    )
    .expect("configuration");
    let schedule = Schedule::new(identity.clone(), configuration).expect("schedule");
    (identity, schedule)
}

#[tokio::test]
async fn concurrent_intake_and_scheduler_ticks_preserve_adherence_and_single_reminder() {
    let store = Arc::new(InMemoryPatientStore::new());
    let store_port: Arc<dyn PatientStore> = store.clone();
    let (overdue_identity, overdue_schedule) =
        schedule_for("overdue-patient", "reminder", 1_000, 1);
    let (intake_identity, intake_schedule) = schedule_for("intake-patient", "tracker", 10_000, 8);
    let register = RegisterSchedule::new(store_port.clone());
    register
        .execute(overdue_schedule)
        .await
        .expect("register overdue schedule");
    register
        .execute(intake_schedule)
        .await
        .expect("register intake schedule");
    let clock = Arc::new(FakeClock::new(Instant::from_millis(1_000)));
    let sink = Arc::new(FakeReminderSink::new());
    let scheduler = SchedulerTick::new(store_port.clone(), clock.clone(), sink.clone());

    clock.advance_to(Instant::from_millis(1_011));
    assert_eq!(scheduler.execute().await.expect("first tick"), 1);

    let submit = Arc::new(SubmitIntake::new(store_port.clone()));
    let mut intake_tasks = Vec::new();
    for dose_index in 0..8_i64 {
        let submit = submit.clone();
        let (patient, medication) = identity_pair(&intake_identity);
        let at = Instant::from_millis(10_005 + dose_index * 100);
        intake_tasks.push(tokio::spawn(async move {
            submit
                .execute(IntakeEvent::new(patient, medication, at))
                .await
        }));
    }
    let mut successful_intakes = 0;
    for task in intake_tasks {
        if task.await.expect("intake task").is_ok() {
            successful_intakes += 1;
        }
    }

    clock.advance_to(Instant::from_millis(10_705));
    let stored = store_port
        .schedule(intake_identity)
        .await
        .expect("schedule query")
        .expect("stored schedule");
    let adherence = stored.adherence_rate(Instant::from_millis(10_705));
    let second_tick = scheduler.execute().await.expect("second tick");
    let overdue_schedule = store_port
        .schedule(overdue_identity)
        .await
        .expect("overdue query")
        .expect("stored overdue schedule");

    assert_eq!(successful_intakes, 8);
    assert_eq!(adherence, 1.0);
    assert_eq!(second_tick, 0);
    assert_eq!(sink.reminders().len(), 1);
    assert_eq!(
        overdue_schedule.adherence_rate(Instant::from_millis(10_705)),
        0.0
    );
}

fn identity_pair(identity: &ScheduleIdentity) -> (PatientId, MedicationName) {
    (identity.patient().clone(), identity.medication().clone())
}
