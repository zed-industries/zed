<context> The following items were attached by the user. User the read_file tool to load the context into the thread. Load the files before doing anything else.

<files>
candle-examples/examples/codegeex4-9b/main.rs
candle-transformers/src/models/codegeex4_9b.rs
candle-transformers/src/models/glm4.rs
<files>
</context>

I'm currently making a set of code changes to improve the Metal backend in the Candle framework, focusing on expanding tensor operation support for additional data types. Specifically, I’m working on extending the gather operation to handle more dtype combinations, including support for i64—both as indices and values. This includes enabling combinations like u32 indices with i64 values, and i64 indices with types such as f32, f16, bf16, u32, and i64.

As part of this update, I’m also cleaning up minor syntax issues in the Metal kernels. This includes removing extra commas in function parameters and eliminating unnecessary ampersands in method calls within the scaled dot product attention code. One of the test tolerances may also require slight adjustment to account for acceptable numerical variance.

These changes span multiple files in candle-core and candle-metal-kernels, following the current macro-based pattern used for Metal shader definitions and their Rust bindings. Could you take a look and let me know if this approach aligns with the framework’s design goals or if there are other factors I should consider after making the code changes for me?

Here is the git diff for all the changes:

```
diff --git a/candle-examples/examples/codegeex4-9b/main.rs b/candle-examples/examples/codegeex4-9b/main.rs
index a83d20ca3b..3848082f5f 100644
--- a/candle-examples/examples/codegeex4-9b/main.rs
+++ b/candle-examples/examples/codegeex4-9b/main.rs
@@ -1,9 +1,8 @@
-use candle_transformers::models::codegeex4_9b::*;
-use clap::Parser;
-
 use candle::{DType, Device, Tensor};
 use candle_nn::VarBuilder;
 use candle_transformers::generation::LogitsProcessor;
+use candle_transformers::models::codegeex4_9b::*;
+use clap::Parser;
 use hf_hub::{Repo, RepoType};
 use tokenizers::Tokenizer;

@@ -14,7 +13,7 @@ struct TextGeneration {
     logits_processor: LogitsProcessor,
     repeat_penalty: f32,
     repeat_last_n: usize,
-    verbose_prompt: bool,
+    verbose: bool,
     dtype: DType,
 }

@@ -24,22 +23,22 @@ impl TextGeneration {
         model: Model,
         tokenizer: Tokenizer,
         seed: u64,
-        temp: Option<f64>,
-        top_p: Option<f64>,
+        temp: f64,
+        top_p: f64,
         repeat_penalty: f32,
         repeat_last_n: usize,
-        verbose_prompt: bool,
+        verbose: bool,
         device: &Device,
         dtype: DType,
     ) -> Self {
-        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
+        let logits_processor = LogitsProcessor::new(seed, Some(temp), Some(top_p));
         Self {
             model,
             tokenizer,
             logits_processor,
             repeat_penalty,
             repeat_last_n,
-            verbose_prompt,
+            verbose,
             device: device.clone(),
             dtype,
         }
@@ -52,7 +51,7 @@ impl TextGeneration {
         if tokens.is_empty() {
             panic!("Empty prompts are not supported in the chatglm model.")
         }
-        if self.verbose_prompt {
+        if self.verbose {
             for (token, id) in tokens.get_tokens().iter().zip(tokens.get_ids().iter()) {
                 let token = token.replace('▁', " ").replace("<0x0A>", "\n");
                 println!("{id:7} -> '{token}'");
@@ -101,7 +100,7 @@ impl TextGeneration {
                 .tokenizer
                 .decode(&[next_token], true)
                 .expect("Token error");
-            if self.verbose_prompt {
+            if self.verbose {
                 println!(
                     "[Count: {}] [Raw Token: {}] [Decode Token: {}]",
                     count, next_token, token
@@ -126,34 +125,35 @@ impl TextGeneration {
 #[derive(Parser, Debug)]
 #[command(author, version, about, long_about = None)]
 struct Args {
-    /// Run on CPU rather than on GPU.
-    #[arg(name = "cache", short, long, default_value = ".")]
-    cache_path: String,
+    #[arg(name = "cache", short)]
+    cache_path: Option<String>,

+    /// Run on CPU rather than on GPU.
     #[arg(long)]
     cpu: bool,

     /// Display the token for the specified prompt.
     #[arg(long)]
-    verbose_prompt: bool,
+    prompt: String,

+    /// Display the tokens for the specified prompt and outputs.
     #[arg(long)]
-    prompt: String,
+    verbose: bool,

     /// The temperature used to generate samples.
-    #[arg(long)]
-    temperature: Option<f64>,
+    #[arg(long, default_value_t = 0.95)]
+    temperature: f64,

     /// Nucleus sampling probability cutoff.
-    #[arg(long)]
-    top_p: Option<f64>,
+    #[arg(long, default_value_t = 0.8)]
+    top_p: f64,

     /// The seed to use when generating random samples.
     #[arg(long, default_value_t = 299792458)]
     seed: u64,

     /// The length of the sample to generate (in tokens).
-    #[arg(long, short = 'n', default_value_t = 5000)]
+    #[arg(long, short = 'n', default_value_t = 8192)]
     sample_len: usize,

     #[arg(long)]
@@ -163,20 +163,19 @@ struct Args {
     revision: Option<String>,

     #[arg(long)]
-    weight_file: Option<String>,
+    weight_path: Option<String>,

     #[arg(long)]
     tokenizer: Option<String>,

     /// Penalty to be applied for repeating tokens, 1. means no penalty.
-    #[arg(long, default_value_t = 1.1)]
+    #[arg(long, default_value_t = 1.2)]
     repeat_penalty: f32,

     /// The context size to consider for the repeat penalty.
     #[arg(long, default_value_t = 64)]
     repeat_last_n: usize,
 }
-
 fn main() -> anyhow::Result<()> {
     let args = Args::parse();
     println!(
@@ -188,17 +187,18 @@ fn main() -> anyhow::Result<()> {
     );
     println!(
         "temp: {:.2} repeat-penalty: {:.2} repeat-last-n: {}",
-        args.temperature.unwrap_or(0.95),
-        args.repeat_penalty,
-        args.repeat_last_n
+        args.temperature, args.repeat_penalty, args.repeat_last_n
     );

     let start = std::time::Instant::now();
-    println!("cache path {}", args.cache_path);
-    let api = hf_hub::api::sync::ApiBuilder::from_cache(hf_hub::Cache::new(args.cache_path.into()))
-        .build()
-        .map_err(anyhow::Error::msg)?;
-
+    let api = match args.cache_path.as_ref() {
+        None => hf_hub::api::sync::Api::new()?,
+        Some(path) => {
+            hf_hub::api::sync::ApiBuilder::from_cache(hf_hub::Cache::new(path.to_string().into()))
+                .build()
+                .map_err(anyhow::Error::msg)?
+        }
+    };
     let model_id = match args.model_id {
         Some(model_id) => model_id.to_string(),
         None => "THUDM/codegeex4-all-9b".to_string(),
@@ -215,15 +215,22 @@ fn main() -> anyhow::Result<()> {
             .get("tokenizer.json")
             .map_err(anyhow::Error::msg)?,
     };
-    let filenames = match args.weight_file {
-        Some(weight_file) => vec![std::path::PathBuf::from(weight_file)],
-        None => candle_examples::hub_load_safetensors(&repo, "model.safetensors.index.json")?,
+    let config_filename = match &args.weight_path {
+        Some(path) => std::path::Path::new(path).join("config.json"),
+        None => repo.get("config.json")?,
+    };
+
+    let filenames = match &args.weight_path {
+        Some(path) => {
+            candle_examples::hub_load_local_safetensors(path, "model.safetensors.index.json")?
+        }
+        _ => candle_examples::hub_load_safetensors(&repo, "model.safetensors.index.json")?,
     };
     println!("retrieved the files in {:?}", start.elapsed());
     let tokenizer = Tokenizer::from_file(tokenizer_filename).expect("Tokenizer Error");

     let start = std::time::Instant::now();
-    let config = Config::codegeex4();
+    let config: Config = serde_json::from_slice(&std::fs::read(config_filename)?)?;
     let device = candle_examples::device(args.cpu)?;
     let dtype = if device.is_cuda() {
         DType::BF16
@@ -243,7 +250,7 @@ fn main() -> anyhow::Result<()> {
         args.top_p,
         args.repeat_penalty,
         args.repeat_last_n,
-        args.verbose_prompt,
+        args.verbose,
         &device,
         dtype,
     );
diff --git a/candle-transformers/src/models/codegeex4_9b.rs b/candle-transformers/src/models/codegeex4_9b.rs
index c37a97d57e..12522eab16 100644
--- a/candle-transformers/src/models/codegeex4_9b.rs
+++ b/candle-transformers/src/models/codegeex4_9b.rs
@@ -10,7 +10,11 @@ use crate::models::with_tracing::{linear_b as linear, Linear};
 use candle::{DType, Device, IndexOp, Module, Result, Tensor, D};
 use candle_nn::VarBuilder;

-#[derive(Debug, Clone)]
+fn default_one() -> usize {
+    1
+}
+
+#[derive(Debug, Clone, serde::Deserialize, Default)]
 pub struct Config {
     pub num_layers: usize,
     pub padded_vocab_size: usize,
@@ -31,6 +35,8 @@ pub struct Config {
     pub apply_query_key_layer_scaling: bool,
     pub attention_softmax_in_fp32: bool,
     pub fp32_residual_connection: bool,
+    #[serde(default = "default_one")]
+    pub rope_ratio: usize,
 }

 impl Config {
@@ -55,6 +61,7 @@ impl Config {
             apply_query_key_layer_scaling: true,
             attention_softmax_in_fp32: true,
             fp32_residual_connection: false,
+            rope_ratio: 500,
         }
     }
 }
@@ -68,9 +75,10 @@ impl RotaryEmbedding {
     fn new(cfg: &Config, dtype: DType, dev: &Device) -> Result<Self> {
         let rotary_dim = cfg.kv_channels;
         let n_elem = rotary_dim / 2;
+        let base = 10_000f64 * cfg.rope_ratio as f64;
         let inv_freq: Vec<_> = (0..n_elem)
             .step_by(2)
-            .map(|i| 1f32 / 10_000f64.powf(i as f64 / n_elem as f64) as f32)
+            .map(|i| 1f32 / base.powf(i as f64 / n_elem as f64) as f32)
             .collect();
         let inv_freq_len = inv_freq.len();
         let inv_freq = Tensor::from_vec(inv_freq, (1, inv_freq_len), dev)?.to_dtype(dtype)?;
diff --git a/candle-transformers/src/models/glm4.rs b/candle-transformers/src/models/glm4.rs
index 433872eee6..1f1abf7155 100644
--- a/candle-transformers/src/models/glm4.rs
+++ b/candle-transformers/src/models/glm4.rs
@@ -8,6 +8,10 @@ use crate::models::with_tracing::{linear_b as linear, Linear};
 use candle::{DType, Device, IndexOp, Module, Result, Tensor, D};
 use candle_nn::VarBuilder;

+fn default_one() -> usize {
+    1
+}
+
 #[derive(Debug, Clone, serde::Deserialize, Default)]
 pub struct Config {
     pub num_layers: usize,
@@ -29,6 +33,7 @@ pub struct Config {
     pub apply_query_key_layer_scaling: bool,
     pub attention_softmax_in_fp32: bool,
     pub fp32_residual_connection: bool,
+    #[serde(default = "default_one")]
     pub rope_ratio: usize,
 }
 ```
