# Palantir-Inspired Design System Guide

This guide outlines how to style your app to match the Palantir-inspired design aesthetic shown in the reference screenshots.

## 🎨 Color Palette

### Primary Colors
- **Accent Green**: `#7dd3c0` - Used for primary actions, badges, and highlights
- **Accent Green Hover**: `#5fb8a6` - Hover states
- **Accent Green Active**: `#4da393` - Active/pressed states

### Neutral Colors
- **Background Primary**: `#ffffff` - Main background
- **Background Secondary**: `#f8f9fa` - Subtle background variation
- **Background Tertiary**: `#e9ecef` - Deeper background

### Text Colors
- **Text Primary**: `#1a1a1a` - Main text
- **Text Secondary**: `#6c757d` - Supporting text
- **Text Tertiary**: `#adb5bd` - Subtle text

### Border Colors
- **Border Light**: `#e9ecef`
- **Border Medium**: `#dee2e6`
- **Border Dark**: `#adb5bd`

## 📦 Component Styles

### 1. Card Components

**Basic Card:**
```html
<div class="card">
  <h3>Card Title</h3>
  <p>Card content goes here</p>
</div>
```

**Elevated Card:**
```html
<div class="card card-elevated">
  <h3>Important Content</h3>
</div>
```

**Asset Card (like screenshot):**
```html
<div class="asset-card">
  <div class="asset-header">ASSET #731</div>
  <div class="asset-info">
    <div class="asset-info-row">
      <span class="asset-label">Engineer</span>
      <span class="asset-value">J. Simmons</span>
    </div>
    <div class="asset-info-row">
      <span class="asset-label">Region</span>
      <span class="asset-value">JP 14</span>
    </div>
    <div class="asset-info-row">
      <span class="asset-label">Location</span>
      <span class="asset-value">39.4142700°</span>
    </div>
  </div>
  <div class="health-score">89</div>
  <div class="asset-label">Health Score</div>
</div>
```

### 2. Buttons

**Primary Button:**
```html
<button class="primary">Start Process</button>
```

**Secondary Button:**
```html
<button>Cancel</button>
```

**Icon Button:**
```html
<button class="icon-only">
  <svg><!-- icon --></svg>
</button>
```

### 3. Badges/Pills (Flow Diagram Style)

```html
<div class="flow-container">
  <div class="flow-items">
    <span class="badge">Records</span>
    <span class="badge">Generates</span>
    <span class="badge">Purchases</span>
    <span class="badge">Delivers</span>
    <span class="badge">Produces</span>
  </div>
</div>
```

### 4. Platform Cards (Environment Cards)

```html
<div class="platform-card">
  <div class="platform-icon">
    <img src="aws-icon.svg" alt="AWS" />
  </div>
  <div class="platform-name">AWS Outposts</div>
</div>
```

## 🎭 Animations

### Fade In
```html
<div class="animate-fade-in">
  Content fades in smoothly
</div>
```

### Slide In
```html
<div class="animate-slide-in">
  Content slides in from left
</div>
```

### Pulse (for status indicators)
```html
<div class="animate-pulse">
  Pulsing indicator
</div>
```

## 🏗️ Layout Patterns

### 1. Isometric Background Pattern

Add this class to create the diagonal grid pattern seen in the screenshots:

```html
<div class="isometric-bg">
  <div class="flow-container">
    <!-- Your content -->
  </div>
</div>
```

### 2. Two-Column Layout (Data & Models)

```html
<div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
  <div class="card">
    <h2>DATA</h2>
    <!-- Data content -->
  </div>
  <div class="card">
    <h2>MODELS</h2>
    <!-- Models content -->
  </div>
</div>
```

### 3. Centered Content with Max Width

```html
<div class="results-content">
  <!-- Content automatically centered with max-width: 1000px -->
</div>
```

## 🎯 Typography Guidelines

### Headings
- Use uppercase for section headers with letter-spacing
- Font weight: 500-600 for emphasis
- Color: `var(--color-text-primary)` or `var(--color-text-secondary)`

### Body Text
- Font size: 0.875rem (14px) for most UI text
- Line height: 1.5-1.6 for readability
- Color: `var(--color-text-secondary)` for supporting text

### Code/Monospace
- Use `'SF Mono', 'Monaco', 'Inconsolata', 'Fira Code', 'Courier New', monospace`
- Background: Dark (#1a1a1a) with light text
- Add custom scrollbar styling

## 🌊 Interaction Patterns

### Hover Effects
- Subtle elevation increase (translateY(-2px to -4px))
- Shadow enhancement (from sm → md → lg)
- Border color darkening
- Smooth transitions (0.2s - 0.3s ease)

### Active/Pressed States
- Slight depression (translateY(1px))
- Shadow reduction
- Immediate feedback

### Focus States
- 2px outline in accent color
- 2px offset for clarity
- Never remove focus indicators

## 📱 Responsive Considerations

### Breakpoints
```css
/* Mobile */
@media (max-width: 640px) {
  .container { padding: 1rem; }
  .card { padding: 1rem; }
}

/* Tablet */
@media (max-width: 1024px) {
  .flow-items { flex-direction: column; }
}
```

## 🎨 Applying to Your Serial App

### Device Status Component
Already updated with:
- Pulsing green indicator for connected state
- Clean card styling with proper shadows
- Accent-colored action button

### Results Page
Already updated with:
- Professional header with border separator
- Dark code output terminal with custom scrollbar
- Info boxes with accent border-left

### Next Steps to Match Screenshots

1. **Add Flow Visualization**
   - Create a flow diagram showing your data pipeline
   - Use badges for each step (Connect → Read → Process → Export)

2. **Create Asset Cards**
   - Show device information in asset card format
   - Display connection stats, port info, baud rate

3. **Add Platform Cards**
   - Show supported platforms or export destinations
   - Use dark cards with radial gradient overlay

4. **Implement Isometric Background**
   - Add to main container or specific sections
   - Creates depth and visual interest

5. **Add Micro-interactions**
   - Button hover effects
   - Card elevation on hover
   - Smooth page transitions

## 🔧 CSS Variables Usage

Always use CSS variables for consistency:

```css
/* Good */
background-color: var(--color-surface);
color: var(--color-text-primary);
box-shadow: var(--shadow-md);

/* Avoid */
background-color: #ffffff;
color: #1a1a1a;
box-shadow: 0 4px 6px rgba(0,0,0,0.1);
```

## 🎬 Animation Best Practices

1. **Keep animations subtle** - 200-500ms duration
2. **Use ease-out for entrances** - Feels more natural
3. **Use ease-in for exits** - Smooth departure
4. **Respect prefers-reduced-motion** - Accessibility first
5. **Animate transform and opacity** - Better performance than width/height

## 📊 Example: Complete Component

```html
<div class="card animate-fade-in">
  <div class="asset-header">DEVICE STATUS</div>
  <div class="asset-info">
    <div class="asset-info-row">
      <span class="asset-label">Port</span>
      <span class="asset-value">COM3</span>
    </div>
    <div class="asset-info-row">
      <span class="asset-label">Baud Rate</span>
      <span class="asset-value">9600</span>
    </div>
    <div class="asset-info-row">
      <span class="asset-label">Status</span>
      <span class="status-connected">Connected</span>
    </div>
  </div>
  <button class="primary">Read Data</button>
</div>
```

This creates a professional, Palantir-style component with proper styling, animations, and interactions.

