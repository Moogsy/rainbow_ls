#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RgbColor {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl RgbColor {
    pub fn get_components_sum(&self) -> usize {
        self.red + self.green + self.blue
    }

    pub fn as_tuple(&self) -> (usize, usize, usize) {
        (self.red, self.green, self.blue)
    }

    pub fn pad_lowest(&mut self, min_rgb_sum: usize) {
        let mut colors_sum: usize = self.get_components_sum();

        // Already good
        if colors_sum > min_rgb_sum {
            return;
        }

        let mut colors: [&mut usize; 3] = [&mut self.red, &mut self.green, &mut self.blue];
        colors.sort_unstable();

        let highest_addable_value: usize = 255 - *colors[2];

        let diff: usize = min_rgb_sum - colors_sum;

        // Just increment all 3 colors simultaneously
        if (highest_addable_value * 3) > diff {
            let to_add: usize = diff / 3;
            for color in colors.iter_mut() {
                **color = **color + to_add;
            }
            return;
        }

        // Increment them by ascending color value
        for color in colors.iter_mut() {
            let potential_new_color: usize = **color + (min_rgb_sum - colors_sum);

            if potential_new_color < 255 {
                **color = potential_new_color;
                return;
            } else {
                let old_color: usize = **color;

                **color = 255;

                colors_sum += (255 - old_color) as usize;
            }
        }
    }
}
