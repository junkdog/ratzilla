/// A type that knows the pixel dimensions of its terminal cells.
///
/// # Physical vs. CSS pixels
///
/// On HiDPI / Retina displays the device pixel ratio (DPR) causes one
/// CSS pixel to map to multiple physical (device) pixels.
///
/// - **Physical pixels** (`cell_size_px`): the actual device pixels
///   occupied by a cell. Useful for pixel-perfect rendering and GPU work.
/// - **CSS pixels** (`cell_size_css_px`): the logical size as seen by
///   the browser layout engine. Useful for DOM positioning, mouse
///   coordinate translation, and canvas drawing.
///
/// [`CanvasBackend`]: crate::CanvasBackend
/// [`DomBackend`]: crate::DomBackend
/// [`WebGl2Backend`]: crate::WebGl2Backend
pub trait CellSized {
    /// Returns the size of a cell in physical (device) pixels as `(width, height)`.
    fn cell_size_px(&self) -> (f64, f64);

    /// Returns the size of a cell in CSS (logical) pixels as `(width, height)`.
    fn cell_size_css_px(&self) -> (f64, f64);
}
