use core::ops::Deref;
use image::{ImageBuffer, Pixel};
use std::sync::mpsc::Sender;
use std::time::Duration;

///Struct to represent a single frame. `P` and `C` are type parameters of the underlying
///[ImageBuffer]
pub struct Frame<P: Pixel, C: Deref<Target = [P::Subpixel]>> {
    timestamp: Duration,
    image: ImageBuffer<P, C>,
}
impl<P: Pixel, C: Deref<Target = [P::Subpixel]>> Frame<P, C> {
    ///Create a new frame
    fn new(timestamp: Duration, image: ImageBuffer<P, C>) -> Frame<P, C> {
        Frame::<P, C> { timestamp, image }
    }
}

///A source of frames (such as a camera)
pub trait FrameSource {
    ///[Pixel] type of the frames which will be returned
    type PixelType: Pixel;
    ///Underlying container holding the pixels which make up a frame
    type ImageContainerType: Deref<Target = [<Self::PixelType as Pixel>::Subpixel]>;
    ///The [FrameStream] that will stream frames from this source
    type Stream: FrameStream;
    ///Set the exposure time
    fn set_exposure(&self, exposure: f64);
    ///Get the exposure time
    fn get_exposure(&self) -> f64;
    ///Set the resolution
    fn set_resolution(&self, resolution: [usize; 2]);
    ///Get the resolution
    fn get_resolution(&self) -> [usize; 2];
    ///Start streaming frames. The frames will be placed into the provided [Sender].
    fn start(
        sender: Sender<Frame<Self::PixelType, Self::ImageContainerType>>,
    ) -> impl FrameStream<PixelType = Self::PixelType, ImageContainerType = Self::ImageContainerType>;
}

///A running stream of frames
pub trait FrameStream {
    ///[Pixel] type of the frames
    type PixelType: Pixel;
    ///Underlying container holding the pixel data
    type ImageContainerType: Deref<Target = [<Self::PixelType as Pixel>::Subpixel]>;
    ///Stop streaming
    fn stop(&self);
    ///Change the destination of the frame stream. New frames will be placed into the provided [Sender]
    fn change_consumer(
        &mut self,
        sender: Sender<
            Frame<<Self as FrameStream>::PixelType, <Self as FrameStream>::ImageContainerType>,
        >,
    );
}
