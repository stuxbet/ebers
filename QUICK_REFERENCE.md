# Quick Reference Guide - Palantir Design System

## 🎨 CSS Classes Reference

### Card Components
```html
<!-- Basic Card -->
<div class="card">Content</div>

<!-- Elevated Card (more shadow) -->
<div class="card card-elevated">Important Content</div>

<!-- Asset Card Structure -->
<div class="card">
  <div class="asset-header">SECTION TITLE</div>
  <div class="asset-info">
    <div class="asset-info-row">
      <span class="asset-label">Label</span>
      <span class="asset-value">Value</span>
    </div>
  </div>
</div>
```

### Buttons
```html
<!-- Primary Button (Accent Green) -->
<button class="primary">Action</button>

<!-- Secondary Button (Default) -->
<button>Cancel</button>

<!-- Icon Only Button -->
<button class="icon-only">🔍</button>
```

### Badges/Pills
```html
<!-- Primary Badge -->
<span class="badge">Label</span>

<!-- Secondary Badge -->
<span class="badge secondary">Label</span>

<!-- Custom Color Badge -->
<span class="badge" style="background-color: var(--color-success);">
  Success
</span>
```

### Status Indicators
```html
<!-- Connected Status -->
<span class="status-connected">Connected</span>

<!-- Disconnected Status -->
<span class="status-disconnected">Disconnected</span>
```

### Animations
```html
<!-- Fade In -->
<div class="animate-fade-in">Content</div>

<!-- Slide In -->
<div class="animate-slide-in">Content</div>

<!-- Pulse (for indicators) -->
<span class="animate-pulse">●</span>

<!-- With Delay -->
<div class="animate-fade-in" style="animation-delay: 200ms;">
  Content
</div>
```

### Layout Components
```html
<!-- Flow Container (with gradient background) -->
<div class="flow-container">
  <div class="flow-items">
    <span class="badge">Step 1</span>
    <span class="badge">Step 2</span>
  </div>
</div>

<!-- Platform Card (dark with gradient) -->
<div class="platform-card">
  <div class="platform-icon">🔌</div>
  <div class="platform-name">USB Serial</div>
</div>

<!-- Results Content (centered, max-width) -->
<div class="results-content">
  Content automatically centered
</div>
```

### Data Display
```html
<!-- Serial Output Terminal -->
<div class="serial-data">
  <pre class="serial-output">
    Terminal output here...
  </pre>
</div>

<!-- Info Box -->
<div class="results-info">
  <p>Information message with accent border</p>
</div>
```

## 🎨 CSS Variables

### Colors
```css
/* Primary Colors */
--color-accent-primary: #7dd3c0
--color-accent-secondary: #5fb8a6
--color-accent-hover: #4da393

/* Backgrounds */
--color-bg-primary: #ffffff
--color-bg-secondary: #f8f9fa
--color-bg-tertiary: #e9ecef
--color-surface: #ffffff

/* Text */
--color-text-primary: #1a1a1a
--color-text-secondary: #6c757d
--color-text-tertiary: #adb5bd

/* Borders */
--color-border-light: #e9ecef
--color-border-medium: #dee2e6
--color-border-dark: #adb5bd

/* Status */
--color-success: #7dd3c0
--color-error: #dc3545
--color-warning: #ffc107
```

### Shadows
```css
--shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05)
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1)
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1)
--shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1)
```

### Usage Example
```css
.my-component {
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-light);
  box-shadow: var(--shadow-md);
}
```

## 📐 Layout Patterns

### Responsive Grid
```html
<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.5rem;">
  <div class="card">Card 1</div>
  <div class="card">Card 2</div>
  <div class="card">Card 3</div>
</div>
```

### Centered Content
```html
<div style="max-width: 1000px; margin: 0 auto;">
  Centered content
</div>
```

### Flex Row with Gap
```html
<div style="display: flex; gap: 1rem; align-items: center;">
  <button>Button 1</button>
  <button>Button 2</button>
</div>
```

## 🎭 Common Patterns

### Header with Action
```html
<div style="display: flex; justify-content: space-between; align-items: center;">
  <div>
    <h1>Title</h1>
    <p>Subtitle</p>
  </div>
  <button class="primary">Action</button>
</div>
```

### Stat Card
```html
<div class="card" style="text-align: center;">
  <div class="asset-label">Label</div>
  <div class="health-score">42</div>
  <div class="asset-label">unit</div>
</div>
```

### Info Row
```html
<div class="asset-info-row">
  <span class="asset-label">Label</span>
  <span class="asset-value">Value</span>
</div>
```

### Badge with Icon
```html
<span class="badge">
  <span class="animate-pulse">●</span>
  Live
</span>
```

## 🎨 Typography Scale

```css
/* Headers */
h1: 2.5rem (40px), font-weight: 300
h2: 1.25rem (20px), font-weight: 500, uppercase
h3: 1rem (16px), font-weight: 500

/* Body */
body: 0.875rem (14px), line-height: 1.5

/* Small */
.asset-label: 0.875rem (14px), uppercase, letter-spacing: 0.05em

/* Large Numbers */
.health-score: 2.5rem (40px), font-weight: 300
```

## 🎯 Best Practices

### 1. Always Use CSS Variables
```css
/* ✅ Good */
color: var(--color-text-primary);

/* ❌ Avoid */
color: #1a1a1a;
```

### 2. Add Animations for Polish
```html
<!-- ✅ Good -->
<div class="card animate-fade-in">Content</div>

<!-- ❌ Missing polish -->
<div class="card">Content</div>
```

### 3. Use Semantic Spacing
```css
/* Consistent spacing scale */
gap: 0.5rem;  /* 8px - tight */
gap: 1rem;    /* 16px - normal */
gap: 1.5rem;  /* 24px - comfortable */
gap: 2rem;    /* 32px - spacious */
```

### 4. Maintain Visual Hierarchy
```html
<!-- Primary action -->
<button class="primary">Save</button>

<!-- Secondary action -->
<button>Cancel</button>
```

### 5. Provide Visual Feedback
```html
<!-- Show loading state -->
<button disabled>
  <span class="spinner"></span>
  Loading...
</button>

<!-- Show status -->
<span class="status-connected">Connected</span>
```

## 📱 Responsive Breakpoints

```css
/* Mobile */
@media (max-width: 640px) {
  /* Stack layouts, increase padding */
}

/* Tablet */
@media (max-width: 1024px) {
  /* Adjust grid columns */
}
```

## 🚀 Quick Start Template

```html
<div class="container">
  <header style="text-align: center; margin-bottom: 3rem;">
    <h1 style="font-size: 2.5rem; font-weight: 300;">
      Your Title
    </h1>
    <p style="color: var(--color-text-secondary);">
      Your subtitle
    </p>
  </header>

  <div class="card card-elevated animate-fade-in">
    <div class="asset-header">SECTION TITLE</div>
    <div class="asset-info">
      <div class="asset-info-row">
        <span class="asset-label">Label</span>
        <span class="asset-value">Value</span>
      </div>
    </div>
    <button class="primary" style="width: 100%; margin-top: 1rem;">
      Action
    </button>
  </div>
</div>
```

---

**This design system provides everything you need to create beautiful, Palantir-inspired interfaces! 🎨**

