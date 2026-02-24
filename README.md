# Amanita

## Checklist

- [ ] Style Builder: create a builder struct for style properties
- [ ] Font Loader: create a mechanism for loading fonts as Font structs
	- Use the loaded font when rendering text affected by the style
- [ ] Element Rendering: re-implement rendering functionality for base elements
- [ ] Review: consider how scroll frame texture rendering will work within the current rendering setup and what changes may need to be made for it to work (maybe push a context used instead of the regular render context?)
- [ ] Pixel Scaling: figure out how to do pixel-perfect rendering and how to upscale the rendered view without distorting pixels
	- e.g. 1 pixel displayed as 4 pixels, fixed aspect ratio centered on screen
- [ ] Button Implementation: add button properties (including actions) and write input handling
- [ ] Button Styling: add styles for button states (hover/click/etc)
	- probably want to have optional alternate style structs for each state (making sure that optional style properties fall back to the main style or an appropriate default)
- [ ] ClippedInputContext
	- same as base input context, but ignores input events outside of its clip area (when applicable, e.g. clicks)
    - this should be used to prevent elements inside a scroll layout from acting on clicks within their areas while they are clipped out of the scroll view
- [ ] Update Cycle
	- update elements in reverse render order
- [ ] Scroll Improvements
  	- configurable limits
    - scaling
    - smoothing
- [ ] Input Events
  	- ensure that elements can consume certain input events to prevent their containing frames from re-using them
- [ ] Element Organization
	- try to extract ordering logic from the render function
    - experiment with static ordering (once right after the ui is initialized)

### Fonts
- Font loading
- Font rendering

### Buttons
- Visual states (style states) (hovered/clicked/disabled/etc)
- Properties (action, disabled, TBD)
- Actions (onclick)

### Pixel Scaling
- Pixel perfect rendering

### Style Features
- Themes - for producing templated styles for elements
- Style builder

### Scroll Frame
~~- Figure out how to render to texture & implications for input handling
	- Can render to a texture and cut off elements outside of the scroll view~~
- Input handling

### Input Variants
- Text input (scary... maybe? i guess there is text measuring)
	- Account for multiline if possible
- Slider input
	- Progress bar (variant)
- Number input
- Checkbox
	- Grouping (?)
- Toggle
- Dropdown (or some kind of multi-select)
	- could alternatively be a dialog or just multiple buttons

### Collapsing Frame
- Renders & measures normally when open (sub-elements are nested in a special container, label is not)
- Renders & measures only label when closed

## Not Important Right Now

### Grid Layout (?)

### Images (?)
- Image loading
- Filters & post-processing effects (?)

