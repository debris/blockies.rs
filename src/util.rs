use pixelate::Color;
use hsl::HSL;

/// Create a buffer of image data where one item corresponds to one square on
/// the blockies identicon
pub fn create_image_data<T, F>(size: usize, mut fill: F) -> Vec<u8>
where
    T: Into<u8>,
    F: FnMut() -> T,
{
    let data_width = size / 2;
    let row_width = size + size % 2;

    let mut data = vec![0; size * row_width];

    for row in data.chunks_mut(row_width) {
        // `right` is going to be 1 item longer if size is odd, but that's fine
        // as we are zipping it with reverse iterator
        let (left, right) = row.split_at_mut(data_width);

        for (left, right) in left.iter_mut().zip(right.iter_mut().rev()) {
            let pixel = fill().into();

            *left = pixel;
            *right = pixel;
        }

        // Technically only have to do this for odd-sized images, but eh
        if let Some(midpoint) = left.last() {
            right[0] = *midpoint;
        }
    }

    data
}

pub fn hsl_to_rgb(hsl: HSL) -> Color {
    let (r, g, b) = hsl.to_rgb();

    Color::Rgb(r, g, b)
}
