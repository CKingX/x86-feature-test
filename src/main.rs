fn main() {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        println!("Testing x86-64 features");

        let fxsr = test_feature(is_x86_feature_detected!("fxsr"));
        println!("fxsr: {fxsr}");

        let sse = test_feature(is_x86_feature_detected!("sse"));
        println!("sse: {sse}");

        let sse2 = test_feature(is_x86_feature_detected!("sse2"));
        println!("sse2: {sse2}");

        println!("");
        println!("Testing x86-64-v2 features (Nehalem/Bulldozer/Silvermont/Jaguar)");

        let cmpxchg16b = test_feature(is_x86_feature_detected!("cmpxchg16b"));
        println!("cmpxchg16b = {cmpxchg16b}");

        let popcnt = test_feature(is_x86_feature_detected!("popcnt"));
        println!("popcnt: {popcnt}");

        let sse3 = test_feature(is_x86_feature_detected!("sse3"));
        println!("sse3: {sse3}");

        let ssse3 = test_feature(is_x86_feature_detected!("ssse3"));
        println!("ssse3: {ssse3}");

        let sse4_1 = test_feature(is_x86_feature_detected!("sse4.1"));
        println!("sse4.1: {sse4_1}");

        let sse4_2 = test_feature(is_x86_feature_detected!("sse4.2"));
        println!("sse4.2: {sse4_2}");

        println!("");
        println!("Testing x86-64-v3 features (Haswell level)");

        let avx = test_feature(is_x86_feature_detected!("avx"));
        println!("avx: {avx}");

        let avx2 = test_feature(is_x86_feature_detected!("avx2"));
        println!("avx2: {avx2}");

        let bmi1 = test_feature(is_x86_feature_detected!("bmi1"));
        println!("bmi1: {bmi1}");

        let bmi2 = test_feature(is_x86_feature_detected!("bmi2"));
        println!("bmi2: {bmi2}");

        let f16c = test_feature(is_x86_feature_detected!("f16c"));
        println!("f16c: {f16c}");

        let fma = test_feature(is_x86_feature_detected!("fma"));
        println!("fma: {fma}");

        let lzcnt = test_feature(is_x86_feature_detected!("lzcnt"));
        println!("lzcnt: {lzcnt}");

        let movbe = test_feature(is_x86_feature_detected!("movbe"));
        println!("movbe: {movbe}");

        let xsave = test_feature(is_x86_feature_detected!("xsave"));
        println!("xsave: {xsave}");

        println!("");
        println!("Testing x86-64-v4 features (AVX512 CPUs)");

        let avx512bw = test_feature(is_x86_feature_detected!("avx512bw"));
        println!("avx512bw: {avx512bw}");

        let avx512cd = test_feature(is_x86_feature_detected!("avx512cd"));
        println!("avx512cd: {avx512cd}");

        let avx512dq = test_feature(is_x86_feature_detected!("avx512dq"));
        println!("avx512dq: {avx512dq}");

        let avx512f = test_feature(is_x86_feature_detected!("avx512f"));
        println!("avx512f: {avx512f}");

        let avx512vl = test_feature(is_x86_feature_detected!("avx512vl"));
        println!("avx512vl: {avx512vl}");
    } else {
        eprintln!("x86 feature dump requires x86 CPU");
    }
}

fn test_feature(feature: bool) -> &'static str {
    if feature {
        "✅"
    } else {
        "❌"
    }
}
