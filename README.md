clone llamacpp to __external folder at the same level as the code


build it using the below shell script
```
cd llama.cpp
mkdir -p build && cd build

cmake .. \
  -DLLAMA_BUILD_EXAMPLES=ON \
  -DLLAMA_BUILD_TESTS=OFF \
  -DLLAMA_CUBLAS=OFF \
  -DLLAMA_METAL=OFF \
  -DGGML_CUDA=ON \
  -DGGML_STATIC=ON \
  -DBUILD_SHARED_LIBS=OFF  # Force static linking for everything

make -j($nproc)
```

download model into a folder and update path in the code in `main.rs`. Only a POC so no CLI interface or config file.
```
n load_gguf_and_print_info() {
    let path = "../../models/llama-2-13b-ensemble-v5.Q4_K_M.gguf"; // <<< Update path
    //let file = GgufFile::load(path).expect("Failed to load .gguf file");
    let (gguf_file, ctx) = GgufFile::load(path).expect("Failed to load");
    ...
```

run it
```
cargo run --release
```

sample output
```
    Finished `release` profile [optimized] target(s) in 0.22s
     Running `target/release/gguf-sys`
[DEBUG] create_tensor start
[DEBUG] create_tensor end
Tensors: 363
Tensor[0]: token_embd.weight
Tensor[0]: token_embd.weight
  Shape: [5120, 32002]
  [not f32 or data not loaded]
Tensor[1]: blk.0.attn_q.weight
Tensor[1]: blk.0.attn_q.weight
  Shape: [5120, 5120]
  [not f32 or data not loaded]
Tensor[2]: blk.0.attn_k.weight
Tensor[2]: blk.0.attn_k.weight
  Shape: [5120, 5120]
  [not f32 or data not loaded]
Tensor[3]: blk.0.attn_v.weight
Tensor[3]: blk.0.attn_v.weight
  Shape: [5120, 5120]
  [not f32 or data not loaded]
Tensor[4]: blk.0.attn_output.weight
Tensor[4]: blk.0.attn_output.weight
  Shape: [5120, 5120]
  [not f32 or data not loaded]
Metadata keys: 19
Metadata[0]: general.architecture
Metadata[1]: general.name
Metadata[2]: llama.context_length
Metadata[3]: llama.embedding_length
Metadata[4]: llama.block_count
```
