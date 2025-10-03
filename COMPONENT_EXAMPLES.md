# Component Implementation Examples

This document shows how to implement Palantir-style components in your Yew/Rust application.

## 🎯 Device Status Card (Enhanced)

```rust
use yew::prelude::*;

#[function_component(DeviceStatusCard)]
pub fn device_status_card() -> Html {
    let is_connected = use_state(|| false);
    
    html! {
        <div class="card animate-fade-in">
            <div class="asset-header">{"DEVICE STATUS"}</div>
            
            <div class="asset-info">
                <div class="asset-info-row">
                    <span class="asset-label">{"Port"}</span>
                    <span class="asset-value">{"COM3"}</span>
                </div>
                <div class="asset-info-row">
                    <span class="asset-label">{"Baud Rate"}</span>
                    <span class="asset-value">{"9600"}</span>
                </div>
                <div class="asset-info-row">
                    <span class="asset-label">{"Status"}</span>
                    if *is_connected {
                        <span class="status-connected">{"Connected"}</span>
                    } else {
                        <span class="status-disconnected">{"Disconnected"}</span>
                    }
                </div>
            </div>
            
            <button class="primary read-results-btn">
                {"Read Data"}
            </button>
        </div>
    }
}
```

## 🔄 Data Flow Visualization

```rust
use yew::prelude::*;

#[function_component(DataFlowDiagram)]
pub fn data_flow_diagram() -> Html {
    let steps = vec![
        "Connect",
        "Detect",
        "Stream",
        "Process",
        "Aggregate",
        "Export CSV"
    ];
    
    html! {
        <div class="flow-container">
            <h2 style="text-align: center; margin-bottom: 2rem; color: var(--color-text-secondary);">
                {"Data Pipeline"}
            </h2>
            <div class="flow-items">
                { for steps.iter().enumerate().map(|(i, step)| {
                    html! {
                        <span 
                            class="badge animate-fade-in" 
                            style={format!("animation-delay: {}ms", i * 100)}
                        >
                            {step}
                        </span>
                    }
                })}
            </div>
        </div>
    }
}
```

## 📊 Statistics Dashboard

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatCardProps {
    pub label: String,
    pub value: String,
    pub unit: Option<String>,
}

#[function_component(StatCard)]
pub fn stat_card(props: &StatCardProps) -> Html {
    html! {
        <div class="card">
            <div class="asset-label">{&props.label}</div>
            <div class="health-score">{&props.value}</div>
            if let Some(unit) = &props.unit {
                <div class="asset-label">{unit}</div>
            }
        </div>
    }
}

#[function_component(StatsDashboard)]
pub fn stats_dashboard() -> Html {
    html! {
        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.5rem; margin: 2rem 0;">
            <StatCard label="Total Readings" value="1,247" unit="samples" />
            <StatCard label="Success Rate" value="98.5" unit="%" />
            <StatCard label="Avg Response" value="45" unit="ms" />
            <StatCard label="Data Points" value="15.2K" unit="points" />
        </div>
    }
}
```

## 🖥️ Platform/Environment Cards

```rust
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Platform {
    pub name: String,
    pub icon: String,
    pub description: String,
}

#[function_component(PlatformGrid)]
pub fn platform_grid() -> Html {
    let platforms = vec![
        Platform {
            name: "Serial USB".to_string(),
            icon: "🔌".to_string(),
            description: "Direct USB connection".to_string(),
        },
        Platform {
            name: "Bluetooth".to_string(),
            icon: "📡".to_string(),
            description: "Wireless BLE".to_string(),
        },
        Platform {
            name: "Network".to_string(),
            icon: "🌐".to_string(),
            description: "TCP/IP connection".to_string(),
        },
        Platform {
            name: "Cloud Export".to_string(),
            icon: "☁️".to_string(),
            description: "Cloud storage".to_string(),
        },
    ];
    
    html! {
        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 1rem; margin: 2rem 0;">
            { for platforms.iter().map(|platform| {
                html! {
                    <div class="platform-card">
                        <div class="platform-icon" style="font-size: 2rem;">
                            {&platform.icon}
                        </div>
                        <div class="platform-name">{&platform.name}</div>
                    </div>
                }
            })}
        </div>
    }
}
```

## 📈 Real-time Data Stream Display

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DataStreamProps {
    pub data: Vec<String>,
    pub is_streaming: bool,
}

#[function_component(DataStreamDisplay)]
pub fn data_stream_display(props: &DataStreamProps) -> Html {
    html! {
        <div class="serial-data">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                <h3 style="margin: 0; color: var(--color-text-primary);">{"Live Data Stream"}</h3>
                if props.is_streaming {
                    <span class="badge" style="background-color: var(--color-success);">
                        <span class="animate-pulse">{"●"}</span>
                        {" Streaming"}
                    </span>
                } else {
                    <span class="badge secondary">{"Idle"}</span>
                }
            </div>
            
            <div class="serial-output">
                { for props.data.iter().map(|line| {
                    html! {
                        <div class="animate-slide-in">{line}</div>
                    }
                })}
            </div>
            
            <div class="results-info" style="margin-top: 1rem;">
                {format!("Received {} data points", props.data.len())}
            </div>
        </div>
    }
}
```

## 🎛️ Control Panel

```rust
use yew::prelude::*;

#[function_component(ControlPanel)]
pub fn control_panel() -> Html {
    let is_connected = use_state(|| false);
    let is_streaming = use_state(|| false);
    
    let toggle_connection = {
        let is_connected = is_connected.clone();
        Callback::from(move |_| {
            is_connected.set(!*is_connected);
        })
    };
    
    let toggle_streaming = {
        let is_streaming = is_streaming.clone();
        Callback::from(move |_| {
            is_streaming.set(!*is_streaming);
        })
    };
    
    html! {
        <div class="card card-elevated">
            <h2 style="margin-top: 0; color: var(--color-text-primary);">{"Control Panel"}</h2>
            
            <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                <button 
                    class={if *is_connected { "primary" } else { "" }}
                    onclick={toggle_connection}
                >
                    {if *is_connected { "Disconnect" } else { "Connect" }}
                </button>
                
                <button 
                    class={if *is_streaming { "primary" } else { "" }}
                    disabled={!*is_connected}
                    onclick={toggle_streaming}
                >
                    {if *is_streaming { "Stop Stream" } else { "Start Stream" }}
                </button>
                
                <button disabled={!*is_connected}>
                    {"Export CSV"}
                </button>
                
                <button disabled={!*is_connected}>
                    {"Clear Data"}
                </button>
            </div>
            
            <div class="connection-message" style="margin-top: 1rem;">
                if *is_connected {
                    if *is_streaming {
                        {"🟢 Connected and streaming data..."}
                    } else {
                        {"🟡 Connected but not streaming"}
                    }
                } else {
                    {"⚪ Not connected"}
                }
            </div>
        </div>
    }
}
```

## 📋 Data Table with Palantir Styling

```rust
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DataRow {
    pub timestamp: String,
    pub value: String,
    pub status: String,
}

#[function_component(DataTable)]
pub fn data_table() -> Html {
    let data = vec![
        DataRow {
            timestamp: "2025-10-03 14:23:45".to_string(),
            value: "23.5°C".to_string(),
            status: "OK".to_string(),
        },
        DataRow {
            timestamp: "2025-10-03 14:23:46".to_string(),
            value: "23.6°C".to_string(),
            status: "OK".to_string(),
        },
        // ... more rows
    ];
    
    html! {
        <div class="card">
            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--color-border-medium);">
                        <th style="text-align: left; padding: 0.75rem; color: var(--color-text-secondary); font-weight: 600; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;">
                            {"Timestamp"}
                        </th>
                        <th style="text-align: left; padding: 0.75rem; color: var(--color-text-secondary); font-weight: 600; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;">
                            {"Value"}
                        </th>
                        <th style="text-align: left; padding: 0.75rem; color: var(--color-text-secondary); font-weight: 600; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;">
                            {"Status"}
                        </th>
                    </tr>
                </thead>
                <tbody>
                    { for data.iter().map(|row| {
                        html! {
                            <tr style="border-bottom: 1px solid var(--color-border-light); transition: background-color 0.2s ease;" 
                                onmouseenter={|e: MouseEvent| {
                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                        let _ = target.style().set_property("background-color", "var(--color-bg-secondary)");
                                    }
                                }}
                                onmouseleave={|e: MouseEvent| {
                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                        let _ = target.style().set_property("background-color", "transparent");
                                    }
                                }}
                            >
                                <td style="padding: 0.75rem; color: var(--color-text-secondary); font-size: 0.875rem;">
                                    {&row.timestamp}
                                </td>
                                <td style="padding: 0.75rem; color: var(--color-text-primary); font-weight: 500; font-size: 0.875rem;">
                                    {&row.value}
                                </td>
                                <td style="padding: 0.75rem;">
                                    <span class="badge" style="background-color: var(--color-success); font-size: 0.75rem; padding: 0.25rem 0.75rem;">
                                        {&row.status}
                                    </span>
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}
```

## 🎨 Usage in Main App

```rust
// In your main app.rs or component file:

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            // Header
            <header style="margin-bottom: 2rem;">
                <h1 style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary); margin: 0;">
                    {"Serial Data Platform"}
                </h1>
                <p style="color: var(--color-text-secondary); margin-top: 0.5rem;">
                    {"Real-time data acquisition and analysis"}
                </p>
            </header>
            
            // Data Flow Visualization
            <DataFlowDiagram />
            
            // Control Panel
            <ControlPanel />
            
            // Stats Dashboard
            <StatsDashboard />
            
            // Live Data Stream
            <DataStreamDisplay data={vec![]} is_streaming={false} />
            
            // Platform Support
            <div style="margin-top: 3rem;">
                <h2 style="color: var(--color-text-secondary); font-size: 1.25rem; margin-bottom: 1rem;">
                    {"Supported Connections"}
                </h2>
                <PlatformGrid />
            </div>
        </div>
    }
}
```

## 💡 Tips for Implementation

1. **Use CSS Variables**: Always reference `var(--color-*)` for consistency
2. **Add Animations**: Use `animate-fade-in`, `animate-slide-in` classes
3. **Hover States**: Cards and buttons should respond to hover
4. **Loading States**: Add pulse animation to loading indicators
5. **Responsive**: Test on different screen sizes
6. **Accessibility**: Maintain focus states and ARIA labels

## 🚀 Next Steps

1. Replace existing components with these styled versions
2. Add smooth page transitions
3. Implement the isometric background pattern
4. Add micro-interactions on button clicks
5. Create a settings panel with the same styling

