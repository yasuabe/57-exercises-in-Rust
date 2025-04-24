/* -----------------------------------
# Ex31: Karvonen Heart Rate
--------------------------------------
- Prompt the user for age and resting heart rate.
- Validate inputs to ensure both are numeric.
- Use the Karvonen formula to calculate target heart rate.
- Loop from 55% to 95% intensity in increments (not hard-coded).
- Display results in a table format.
*/
use exercises_for_programmer::utils::std_util::read_parsed;
use std::num::NonZeroU32;

fn read_non_zero_u32(prompt: &str) -> NonZeroU32 {
    read_parsed::<NonZeroU32>(prompt, "Sorry. That's not a valid input.")
}
fn gen_heart_rate_func(age: NonZeroU32, resting_hr: NonZeroU32) -> Box<dyn Fn(f64) -> u32> {
    let age        = age.get() as f64;
    let resting_hr = resting_hr.get() as f64;

    Box::new(move |intensity| (((220.0 - age) - resting_hr) * intensity + resting_hr) as u32)
}
fn display_rate_table(compute_heart_rate: Box<dyn Fn(f64) -> u32>) {
    println!("Intensity | Rate");
    println!("----------|-----");
    (55..= 95).step_by(5).for_each(| intensity | {
        let target_hr = compute_heart_rate(intensity as f64 / 100.0);
        println!("{:<10}| {:>1} bpm", format!("{}%", intensity), target_hr);
    });
}
fn main() {
    let resting_hr = read_non_zero_u32("Resting Pulse: ");
    let age        = read_non_zero_u32("Age: ");

    let heart_rate_func = gen_heart_rate_func(age, resting_hr);
    display_rate_table(heart_rate_func)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_heart_rate_func() {
        let rate = NonZeroU32::new(65).unwrap();
        let age  = NonZeroU32::new(22).unwrap();
        let func = gen_heart_rate_func(age, rate);

        assert_eq!(138, func(0.55));
        assert_eq!(191, func(0.95));
    }
}