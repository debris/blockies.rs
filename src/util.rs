use crate::Rgb;
use hsl::HSL;
use std::iter::repeat;

/// Create a buffer of image data where one item corresponds to one square on
/// the blockies identicon
pub fn create_image_data<T, F>(size: usize, mut fill: F) -> Vec<u8>
where
    T: Default + Into<u8>,
    F: FnMut() -> T,
{
    let data_width = size / 2;
    let row_width = size + size % 2;

    let mut data = vec![T::default().into(); size * row_width];

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

/// Expands the image data, scaling up each square to scale * scale pixel size and
/// storing each pixel as 2 bits
pub fn rasterize(data: &[u8], row_width: usize, row_height: usize, scale: usize, depth: usize) -> Vec<u8> {
    // const DEPTH: usize = 2;
    // const PIXELS_IN_BYTE: usize = 8 / DEPTH;
    let pixels_in_byte = 8 / depth;

    let width = row_width * scale;
    let height = row_height * scale;

    let bytes_per_row = width / pixels_in_byte + if (width % pixels_in_byte) == 0 { 0 } else { 1 };

    let mut raster = vec![0; bytes_per_row * height];

    let chunk_size = bytes_per_row * scale;

    for (chunk, row) in raster.chunks_mut(chunk_size).zip(data.chunks(row_width)) {
        let (first_row, chunk) = chunk.split_at_mut(bytes_per_row);

        let mut pixels = row.iter().flat_map(|pixel| repeat(*pixel).take(scale));

        // Rasterize the first row of pixels
        for byte in first_row.iter_mut() {
            for (idx, pixel) in (&mut pixels).take(pixels_in_byte).enumerate() {
                *byte |= pixel << (8 - depth) - (idx * depth);
            }
        }

        // Now repeat it until we've filled up the whole scale-tall section
        for row in chunk.chunks_mut(bytes_per_row) {
            row.copy_from_slice(first_row);
        }
    }

    raster
}

pub fn hsl_to_rgb(hsl: HSL) -> Rgb {
    let (r, g, b) = hsl.to_rgb();

    [r, g, b]
}
