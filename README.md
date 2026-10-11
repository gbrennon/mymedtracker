# Medtrack

Medtrack is a concurrent medication-adherence tracker implemented as a Rust Cargo workspace.
It records intake events, calculates adherence, and emits one reminder for each overdue dose.

## Architecture

The workspace follows hexagonal architecture. Dependencies point inward from adapters to
application ports and domain entities.

- `crates/domain` contains pure value objects, the `Dose` entity, the `Schedule` aggregate,
  and domain errors. It has no async runtime or infrastructure dependency.
- `crates/application` contains use cases and outbound ports for clocks, schedule storage,
  and reminder delivery. The `PatientStore` port exposes atomic intake recording and atomic
  overdue-reminder claiming.
- `crates/infrastructure` implements the ports with a Tokio `RwLock` in-memory store,
  a system clock, a stdout reminder sink, and an `mpsc` intake bus.
- `crates/app` is the Tokio composition root and bounded demonstration binary.

The store key is a patient and medication identity. Intake recording holds the store write
lock across lookup and domain mutation. Reminder claiming holds the same lock while each
schedule claims its overdue doses. These operations prevent lost adherence updates and
duplicate reminders under concurrency.

## Async model

The demo starts an intake consumer task, several concurrent producer tasks, and a scheduler
task. Producers submit `IntakeEvent` values to a bounded Tokio `mpsc` channel. The consumer
passes each event to `SubmitIntake`, while the scheduler uses a Tokio interval to call
`SchedulerTick`. Each scheduler tick obtains the injected clock value, atomically claims
overdue reminders, and sends messages to the injected `ReminderSink`.

The scheduler and intake paths share the concurrency-safe store. The domain aggregate owns
state transitions: callers record intake through `Schedule::record_intake`, and the scheduler
claims reminders through `Schedule::claim_overdue_reminders`.

## Run

Build the entire workspace:

```text
cargo build --workspace
```

Run the bounded demo:

```text
cargo run -p medtrack-app
```

Run all unit and integration tests:

```text
cargo test --workspace
```

The scheduler integration test injects a fake clock and fake reminder sink. It advances virtual
time without wall-clock sleeps, submits eight intake events concurrently, checks full adherence,
and verifies that repeated scheduler ticks produce exactly one reminder for one overdue dose.
