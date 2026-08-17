# Color quantization library
This library provides a color quantizer based on the [NEUQUANT](https://scientificgems.wordpress.com/stuff/neuquant-fast-high-quality-image-quantization/)
quantization algorithm by Anthony Dekker.

### Usage

    let data = vec![0; 40];
    let nq = color_quant::NeuQuant::new(10, 256, &data);
    let indixes: Vec<u8> = data.chunks(4).map(|pix| nq.index_of(pix) as u8).collect();
    let color_map = nq.color_map_rgba();

