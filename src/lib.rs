use image::DynamicImage;
use std::sync::mpsc::Sender;
use std::time::Duration;

//re-export image to our callers
pub use image;

///Struct to represent a single frame. `P` and `C` are type parameters of the underlying
///[ImageBuffer]
pub struct Frame {
    timestamp: Duration,
    image: DynamicImage,
}
impl Frame {
    ///Create a new `Frame`
    pub fn new(timestamp: Duration, image: DynamicImage) -> Frame {
        Frame { timestamp, image }
    }
    ///Get the image associated with this `Frame`. This method will consume the `Frame`
    ///struct to avoid copying image data
    pub fn to_image(self) -> DynamicImage {
        self.image
    }
    ///Get the image associated with this `Frame`
    pub fn get_timestamp(&self) -> Duration {
        self.timestamp
    }
}

///A source of frames (such as a camera)
pub trait FrameSource {
    ///The [FrameStream] that will stream frames from this source
    type Stream: FrameStream;
    ///Set the exposure time
    fn set_exposure(&mut self, exposure: f64);
    ///Get the exposure time
    fn get_exposure(&self) -> f64;
    ///Set the resolution
    fn set_resolution(&mut self, resolution: [usize; 2]);
    ///Get the resolution
    fn get_resolution(&self) -> [usize; 2];
    ///Start streaming frames. The frames will be placed into the provided [Sender].
    fn start(&self, sender: Sender<Frame>) -> Self::Stream;
}

///A running stream of frames
pub trait FrameStream {
    ///Stop streaming
    fn stop(self);
    ///Change the destination of the frame stream. New frames will be placed into the provided [Sender]
    fn change_consumer(&mut self, sender: Sender<Frame>);
}
