use symphonia_codec_aac::{AacDecoder, AdtsReader};
use symphonia_core::codecs::{CodecParameters, Decoder, DecoderOptions, CODEC_TYPE_AAC};
use symphonia_core::errors;
use symphonia_core::formats::{FormatOptions, FormatReader};
use symphonia_core::io::MediaSourceStream;

fn test_decode(data: Vec<u8>) -> symphonia_core::errors::Result<()> {
    let data = std::io::Cursor::new(data);

    let source = MediaSourceStream::new(Box::new(data), Default::default());

    let mut reader = AdtsReader::try_new(source, &FormatOptions::default())?;

    let mut decoder = AacDecoder::try_new(
        CodecParameters::new().for_codec(CODEC_TYPE_AAC),
        &DecoderOptions::default(),
    )?;

    loop {
        let packet = reader.next_packet()?;
        let _ = decoder.decode(&packet);
    }
}

#[test]
fn invalid_channels_aac() {
    let file = vec![
        0xff, 0xf1, 0xaf, 0xce, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xfb,
        0xaf,
    ];

    let err = test_decode(file).unwrap_err();

    assert!(matches!(err, errors::Error::Unsupported(_)));
}
