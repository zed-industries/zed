# Telemetry in Zed

Zed collects anonymous telemetry data to help the team understand how people are using the application and to see what sort of issues they are experiencing.

## Configuring Telemetry Settings

You have full control over what data is sent out by Zed. To enable or disable some or all telemetry types, open your `settings.json` file via {#action zed::OpenSettings}({#kb zed::OpenSettings}) from the command palette.

Insert and tweak the following:

```json
"telemetry": {
    "diagnostics": false,
    "metrics": false
},
```

The telemetry settings can also be configured via the welcome screen, which can be invoked via the {#action workspace::Welcome} action in the command palette.

## Dataflow

Telemetry is sent from the application to our servers. Data is proxied through our servers to enable us to easily switch analytics services. We currently use:

- [Sentry](https://sentry.io): Crash-monitoring service - stores diagnostic events
- [Snowflake](https://snowflake.com): Data warehouse - stores both diagnostic and metric events
- [Hex](https://www.hex.tech): Dashboards and data exploration - accesses data stored in Snowflake
- [Amplitude](https://www.amplitude.com): Dashboards and data exploration - accesses data stored in Snowflake

## Types of Telemetry

### Diagnostics

Crash reports consist of a [minidump](https://learn.microsoft.com/en-us/windows/win32/debug/minidump-files) and some extra debug information. Reports are sent on the first application launch after the crash occurred. We've built dashboards that allow us to visualize the frequency and severity of issues experienced by users. Having these reports sent automatically allows us to begin implementing fixes without the user needing to file a report in our issue tracker. The plots in the dashboards also give us an informal measurement of the stability of Zed.

You can see what extra data is sent alongside the minidump in the `Panic` struct in [crates/telemetry_events/src/telemetry_events.rs](https://github.com/zed-industries/zed/blob/main/crates/telemetry_events/src/telemetry_events.rs) in the Zed repo. You can find additional information in the [Debugging Crashes](./development/debugging-crashes.md) documentation.

### Client-Side Usage Data {#client-metrics}

To improve Zed and understand how it is being used in the wild, Zed optionally collects usage data like the following:

- (a) file extensions of opened files;
- (b) features and tools You use within the Editor;
- (c) project statistics (e.g., number of files); and
- (d) frameworks detected in Your projects

Usage Data does not include any of Your software code or sensitive project details. Metric events are reported over HTTPS, and requests are rate-limited to avoid using significant network bandwidth.

Usage Data is associated with a secure random telemetry ID which may be linked to Your email address. This linkage currently serves two purposes: (1) it allows Zed to analyze usage patterns over time while maintaining Your privacy; and (2) it enables Zed to reach out to specific user groups for feedback and improvement suggestions.

You can audit the metrics data that Zed has reported by running the command {#action zed::OpenTelemetryLog} from the command palette, or clicking `Help > View Telemetry Log` in the application menu.

You can see the full list of the event types and exactly the data sent for each by inspecting the `Event` enum and the associated structs in [crates/telemetry_events/src/telemetry_events.rs](https://github.com/zed-industries/zed/blob/main/crates/telemetry_events/src/telemetry_events.rs) in the Zed repository.

### Server-Side Usage Data {#metrics}

When using Zed's hosted services, we may collect, generate, and Process data to allow us to support users and improve our hosted offering. Examples include metadata around rate limiting and billing metrics/token usage. Zed does not persistently store user content or use user content to evaluate and/or improve our AI features, unless it is explicitly shared with Zed, and we have a zero-data retention agreement with Anthropic.

You can see more about our stance on data collection (and that any prompt data shared with Zed is explicitly opt-in) at [AI Improvement](./ai/ai-improvement.md).

## Concerns and Questions

If you have concerns about telemetry, please feel free to [open an issue](https://github.com/zed-industries/zed/issues/new/choose).
