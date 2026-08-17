#[derive(Debug, Clone)]
pub struct YUVFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: i32,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: i32,
}

#[derive(Debug, Clone)]
pub struct RGBFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RGB8Frame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct RGBxFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct XBGRFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BGRxFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BGRFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BGRAFrame {
    pub display_time: u64,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FrameType {
    #[default]
    YUVFrame,
    BGR0,
    RGB, // Prefer BGR0 because RGB is slower
    BGRAFrame,
}

#[derive(Debug, Clone)]
pub enum Frame {
    YUVFrame(YUVFrame),
    RGB(RGBFrame),
    RGBx(RGBxFrame),
    XBGR(XBGRFrame),
    BGRx(BGRxFrame),
    BGR0(BGRFrame),
    BGRA(BGRAFrame),
}

pub enum FrameData<'a> {
    NV12(&'a YUVFrame),
    BGR0(&'a [u8]),
}

pub fn remove_alpha_channel(frame_data: Vec<u8>) -> Vec<u8> {
    let width = frame_data.len();
    let width_without_alpha = (width / 4) * 3;

    let mut data: Vec<u8> = vec![0; width_without_alpha];

    for (src, dst) in frame_data.chunks_exact(4).zip(data.chunks_exact_mut(3)) {
        dst[0] = src[0];
        dst[1] = src[1];
        dst[2] = src[2];
    }

    data
}

pub fn convert_bgra_to_rgb(frame_data: Vec<u8>) -> Vec<u8> {
    let width = frame_data.len();
    let width_without_alpha = (width / 4) * 3;

    let mut data: Vec<u8> = vec![0; width_without_alpha];

    for (src, dst) in frame_data.chunks_exact(4).zip(data.chunks_exact_mut(3)) {
        dst[0] = src[2];
        dst[1] = src[1];
        dst[2] = src[0];
    }

    data
}

pub fn get_cropped_data(data: Vec<u8>, cur_width: i32, height: i32, width: i32) -> Vec<u8> {
    if data.len() as i32 != height * cur_width * 4 {
        data
    } else {
        let mut cropped_data: Vec<u8> = vec![0; (4 * height * width).try_into().unwrap()];
        let mut cropped_data_index = 0;

        for (i, item) in data.iter().enumerate() {
            let x = i as i32 % (cur_width * 4);
            if x < (width * 4) {
                cropped_data[cropped_data_index] = *item;
                cropped_data_index += 1;
            }
        }
        cropped_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_alpha_channel() {
        assert_eq!(remove_alpha_channel(vec![1, 2, 3, 0]), vec![1, 2, 3]);
        assert_eq!(
            remove_alpha_channel(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![1, 2, 3, 5, 6, 7]
        );
    }

    #[test]
    fn test_convert_bgra_to_rgb() {
        assert_eq!(convert_bgra_to_rgb(vec![1, 2, 3, 0]), vec![3, 2, 1]);
        assert_eq!(
            convert_bgra_to_rgb(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![3, 2, 1, 7, 6, 5]
        );
    }

    macro_rules! rgba {
        ($n:expr) => {
            &mut vec![$n, $n, $n, $n]
        };
    }

    #[test]
    pub fn test_get_cropped_data() {
        let mut data: Vec<u8> = Vec::new();
        for i in 1..=9 {
            data.append(rgba!(i));
        }
        let mut expected: Vec<u8> = Vec::new();
        expected.append(rgba!(1));
        expected.append(rgba!(2));
        expected.append(rgba!(4));
        expected.append(rgba!(5));
        expected.append(rgba!(7));
        expected.append(rgba!(8));
        assert_eq!(get_cropped_data(data, 3, 3, 2), expected)
    }
}
