# x86 Feature Levels
This tool determines which [x86 microarchitectural level](https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels) is supported by your CPU. Must be using an x86-64 CPU.

## Usage
Run the executable in terminal in Linux or in Command Prompt or Powershell in Windows. If you downloaded the Linux binary, you may need to run `./chmod +x {name of file}`

Here is an example output:
```
Testing x86-64 features
fxsr: ✅
sse: ✅
sse2: ✅

Testing x86-64-v2 features (Nehalem/Bulldozer/Silvermont/Jaguar)
cmpxchg16b = ✅
popcnt: ✅
sse3: ✅
ssse3: ✅
sse4.1: ✅
sse4.2: ✅

Testing x86-64-v3 features (Haswell level)
avx: ✅
avx2: ✅
bmi1: ✅
bmi2: ✅
f16c: ✅
fma: ✅
lzcnt: ✅
movbe: ✅
xsave: ✅

Testing x86-64-v4 features (AVX512 CPUs)
avx512bw: ❌
avx512cd: ❌
avx512dq: ❌
avx512f: ❌
avx512vl: ❌
```

## Build Guide
You need to install the [Rust toolchain](https://www.rust-lang.org/tools/install). As an example, on Linux, the following comamnd will install the Rust toolchain:
```
curl https://sh.rustup.rs -sSf | sh
```

Then, it can be built with 
```
git clone https://github.com/CKingX/x86-feature-test
cd ./x86-feature-test
cargo build --release
```
The resulting binary is stored in `./target/release/x86featuretest`

It can be installed with:
```
cargo install --path {path}
```

Since Rust 1.78 (which requires Windows 10), additional features are enabled out of the box including SSE3. This means the tool will report SSE3 compatibility without testing it. As all Windows 10 compatible CPUs support this, this should not be a problem. This is a similar case on macOS (though introduced in earlier versions of Rust). However, if you want to change this for Windows, you can change Cargo's config.toml with (if you have existing rustflags, you can append to it instead):
```
rustflags = ["-C", "target-features=\"-sse3\""]
```

for macOS on a Hackintosh, config.toml can be:
```
rustflags = ["-C", "target-cpu=x86-64"]
```

In addition, the existing Windows binary on the GitHub Releases page does runtime checks for these instructions so you are good to go.
## License
This tool is licensed under MIT Expat.