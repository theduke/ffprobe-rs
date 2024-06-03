use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ratio::Ratio, streams::StreamTags};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Tags specific for video
pub struct VideoTags {
    #[serde(flatten)]
    pub tags: StreamTags,
    pub filename: Option<String>,
    pub mimetype: Option<String>,
    #[serde(flatten)]
    /// Unknown tags
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
/// Stream of type video
pub struct VideoStream {
    /// width of video
    pub width: i64,
    /// height of video
    pub height: i64,
    /// height before cropping
    /// https://superuser.com/questions/1523944/whats-the-difference-between-coded-width-and-width-in-ffprobe
    pub coded_height: i64,
    /// width before cropping
    /// https://superuser.com/questions/1523944/whats-the-difference-between-coded-width-and-width-in-ffprobe
    pub coded_width: i64,
    /// ratio of the width to the height of individual pixels in the video. It describes how the pixels are stored in the video file
    pub sample_aspect_ratio: Option<Ratio>,
    /// ratio of the width to the height of the video as it is intended to be viewed. This aspect ratio dictates the shape of the displayed image on the screen.
    pub display_aspect_ratio: Option<Ratio>,
    ///  This specifies the number of bits used to represent each component of the pixel. For example, in an 8-bit raw sample, each color component (e.g., red, green, and blue in an RGB format) is represented by 8 bits, allowing 256 different levels per component.
    pub bits_per_raw_sample: Option<String>,
    /// Location of chroma samples in the video (e.g., left, center).
    /// Chroma samples refer to the color information in a video image. In video and image processing, the image is typically represented in a color space where the luminance (brightness) and chrominance (color) are separated. The chrominance components (chroma) are often sub-sampled to reduce the amount of data that needs to be processed and stored.
    pub chroma_location: Option<String>,
    /// Indicates the presence of closed captions in the video. (0/1)
    /// Closed captioning (CC) and subtitling are both processes of displaying text on a television, video screen, or other visual display to provide additional or interpretive information. Both are typically used as a transcription of the audio portion of a program as it occurs (either verbatim or in edited form), sometimes including descriptions of non-speech elements
    pub closed_captions: i64,
    /// Long name of the codec used for the video stream.
    pub codec_long_name: String,
    /// Short name of the codec used for the video stream.
    /// Example: h264
    pub codec_name: String,
    /// Indicates the color primaries used in the video (e.g., BT.709).
    pub color_primaries: Option<String>,
    // Indicates the color range used in the video (e.g., full, limited).
    pub color_range: Option<String>,
    /// Indicates the color space used in the video (e.g., YUV, RGB).
    pub color_space: Option<String>,
    /// Indicates the color transfer characteristic used in the video (e.g., BT.709).
    pub color_transfer: Option<String>,
    /// Order in which fields are interlaced (e.g., top first, bottom first).
    /// Order in which fields are interlaced in the video.
    /// Interlaced video consists of two fields per frame, each containing a subset of the lines in the frame.
    /// Field order determines how these fields are displayed:
    /// - `Top field first`: The first field contains the topmost lines (odd lines), followed by the second field with the even lines.
    /// - `Bottom field first`: The first field contains the bottommost lines (even lines), followed by the second field with the odd lines.
    /// - `Progressive`: The video is not interlaced; each frame is displayed as a whole.
    /// - `Unknown`: The field order is not specified.
    pub field_order: Option<String>,
    /// Indicates the presence of film grain in the video.
    pub film_grain: i64,
    /// Number of B-frames between I-frames and P-frames in the video.
    /// MPEG-2 includes three basic types of coded frames: intra-coded frames (I-frames), predictive-coded frames (P-frames), and bidirectionally-predictive-coded frames (B-frames).
    /// An I-frame is a separately-compressed version of a single uncompressed (raw) frame. The coding of an I-frame takes advantage of spatial redundancy and of the inability of the eye to detect certain changes in the image. Unlike P-frames and B-frames, I-frames do not depend on data in the preceding or the following frames, and so their coding is very similar to how a still photograph would be coded (roughly similar to JPEG picture coding). Briefly, the raw frame is divided into 8 pixel by 8 pixel blocks. The data in each block is transformed by the discrete cosine transform (DCT). The result is an 8×8 matrix of coefficients that have real number values. The transform converts spatial variations into frequency variations, but it does not change the information in the block; if the transform is computed with perfect precision, the original block can be recreated exactly by applying the inverse cosine transform (also with perfect precision). The conversion from 8-bit integers to real-valued transform coefficients actually expands the amount of data used at this stage of the processing, but the advantage of the transformation is that the image data can then be approximated by quantizing the coefficients. Many of the transform coefficients, usually the higher frequency components, will be zero after the quantization, which is basically a rounding operation. The penalty of this step is the loss of some subtle distinctions in brightness and color. The quantization may either be coarse or fine, as selected by the encoder. If the quantization is not too coarse and one applies the inverse transform to the matrix after it is quantized, one gets an image that looks very similar to the original image but is not quite the same. Next, the quantized coefficient matrix is itself compressed. Typically, one corner of the 8×8 array of coefficients contains only zeros after quantization is applied. By starting in the opposite corner of the matrix, then zigzagging through the matrix to combine the coefficients into a string, then substituting run-length codes for consecutive zeros in that string, and then applying Huffman coding to that result, one reduces the matrix to a smaller quantity of data. It is this entropy coded data that is broadcast or that is put on DVDs. In the receiver or the player, the whole process is reversed, enabling the receiver to reconstruct, to a close approximation, the original frame.
    /// The processing of B-frames is similar to that of P-frames except that B-frames use the picture in a subsequent reference frame as well as the picture in a preceding reference frame. As a result, B-frames usually provide more compression than P-frames. B-frames are never reference frames in MPEG-2 Video.
    /// Typically, every 15th frame or so is made into an I-frame. P-frames and B-frames might follow an I-frame like this, IBBPBBPBBPBB(I), to form a Group of Pictures (GOP); however, the standard is flexible about this. The encoder selects which pictures are coded as I-, P-, and B-frames.
    pub has_b_frames: i64,
    /// Indicates whether the video stream is in AVC format.
    /// Advanced Video Coding (AVC), also referred to as H.264 or MPEG-4 Part 10, is a video compression standard based on block-oriented, motion-compensated coding.[2] It is by far the most commonly used format for the recording, compression, and distribution of video content, used by 91% of video industry developers as of September 2019.[3][4] It supports a maximum resolution of 8K UHD.[5][6]
    pub is_avc: Option<String>,
    /// Level of the codec profile used for the video stream.
    /// TODO: explain
    pub level: i64,
    /// Size of the NAL (Network Abstraction Layer) units in the video stream.
    pub nal_length_size: Option<String>,
    /// Pixel format used in the video stream (e.g., yuv420p).
    pub pix_fmt: Option<String>,
    /// Profile of the codec used for the video stream (e.g., Main, High).
    pub profile: Option<String>,
    /// Duration of the video stream in timestamp units.
    pub duration_ts: Option<u64>,
    /// Number of reference frames in the video stream.
    /// TODO: Explain
    pub refs: i64,
    /// Metadata tags associated with the video stream.
    pub tags: Option<VideoTags>,
    /// Bit rate of the video stream.
    /// The bit_rate represents the number of bits that are processed per unit of time in the video stream. It is a measure of the video stream's data rate, indicating how much data is encoded for each second of video.
    pub bit_rate: Option<String>,
    /// boolean
    /// divx_packed is a codec-specific property related to the DivX codec. DivX is a popular video codec used for compressing and decompressing digital video. The divx_packed property likely indicates whether the video stream is packed in a particular way specific to the DivX codec.
    divx_packed: Option<String>,
    /// boolean
    /// the quarter_sample option is used to analyze only one out of every four frames of a video when gathering information about it. This option can be handy when you want to speed up the analysis process or when you're not interested in examining every single frame in detail. By analyzing a quarter of the frames, you can get a general overview of the video's properties without processing unnecessary data.
    quarter_sample: Option<String>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    codec_type: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    start_time: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    duration: Option<serde_json::Value>,
}
