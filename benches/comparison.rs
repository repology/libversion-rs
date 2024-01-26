use criterion::{criterion_group, criterion_main, Criterion};

use libversion::*;

fn comparison_benchmark(c: &mut Criterion) {
    c.bench_function("compare numeric", |b| b.iter(|| {
        version_compare2(
            "1.2.3.4.5.6.7.8.9.10.11.12.13.14.15.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30",
            "1.2.3.4.5.6.7.8.9.10.11.12.13.14.15.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30",
        )
    }));

    c.bench_function("compare alphabetic short", |b| b.iter(|| {
        version_compare2(
            "a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z",
            "a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z",
        )
    }));

    c.bench_function("compare letter suffix", |b| b.iter(|| {
        version_compare2(
            "1a.2b.3c.4d.5e.6f.7g.8h.9i.10j.11k.12l.13m.14n.15o.16p.17q.18r.19s.20t.21u.22v.23w.23x.24y.25z",
            "1a.2b.3c.4d.5e.6f.7g.8h.9i.10j.11k.12l.13m.14n.15o.16p.17q.18r.19s.20t.21u.22v.23w.23x.24y.25z"
        )
    }));

    c.bench_function("compare alphabetic long", |b| b.iter(|| {
        version_compare2(
            "alpha.beta.pre.prerelease.postrelease.patch.errata.pl.p.alpha.beta.pre.prerelease.post.postrelease.patch.errata.pl.p",
            "alpha.beta.pre.prerelease.postrelease.patch.errata.pl.p.alpha.beta.pre.prerelease.post.postrelease.patch.errata.pl.p",
        )
    }));

    c.bench_function("compare mixed", |b| b.iter(|| {
        version_compare2(
            "1.2.alpha.3.beta.4.pre.5.post.6.patch.7.8a.9b.10c.11.a.12.b.13.c.d.e.f.0.0.0.0.0.0",
            "1.2.alpha.3.beta.4.pre.5.post.6.patch.7.8a.9b.10c.11.a.12.b.13.c.d.e.f.0",
        )
    }));
}

criterion_group!(benches, comparison_benchmark);
criterion_main!(benches);
