// Write function bmi that calculates body mass index (bmi = weight / height2).

// if bmi <= 18.5 return "Underweight"

// if bmi <= 25.0 return "Normal"

// if bmi <= 30.0 return "Overweight"

// if bmi > 30 return "Obese"

fn bmi(weight: u32, height: f32) -> &'static str {
  let result = weight as f32 / height.powf(2.0);
    if result <= 18.5 {
        "Underweight"
    } else if result <= 25.0 {
        "Normal"
    } else if result <= 30.0 {
        "Overweight"
    } else {
        "Obese"
    }
}

fn bmi(weight: u32, height: f32) -> &'static str {
    let index = weight as f32 / height.powi(2);
    match index {
        index if index <= 18.5 => "Underweight",
        index if index <= 25.0 => "Normal",
        index if index <= 30.0 => "Overweight",
        _ => "Obese"
    }
}
