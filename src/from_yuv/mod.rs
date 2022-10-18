pub mod yuv2rgba;

pub trait YUVToRGBAConversionContext {
    fn new(width: u32, height: u32) -> Self; 
    fn convert(
        &mut self,
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    );
}
