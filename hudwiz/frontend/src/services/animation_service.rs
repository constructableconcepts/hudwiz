use web_sys::Element;
use gloo_timers::callback::Timeout;

/// Applies a one-off CSS animation to a DOM element.
///
/// It adds the specified animation class to trigger the animation and then
/// removes it after the animation completes to allow for it to be re-triggered
/// in the future.
///
/// # Arguments
///
/// * `element` - The `Element` to apply the animation to.
/// * `animation_class` - The CSS class that defines the animation.
/// * `duration_ms` - The duration of the animation in milliseconds. This is used
///   to schedule the removal of the class.
pub fn apply_animation(element: Element, animation_class: &'static str, duration_ms: u32) {
    // Add the class to trigger the animation
    element.class_list().add_1(animation_class).unwrap();

    // Set a timeout to remove the class after the animation is done
    let timeout = Timeout::new(duration_ms, move || {
        element.class_list().remove_1(animation_class).unwrap();
    });

    // The timeout handle is dropped here, which is fine for a one-off animation.
    // The browser will still execute the timeout.
    timeout.forget();
}