export default function Home() {
  return (
    <div className="min-h-screen bg-[radial-gradient(circle_at_top,_#fff7ed_0%,_#fef3c7_35%,_#f8fafc_70%,_#ecfeff_100%)]">
      <main className="mx-auto flex min-h-screen w-full max-w-6xl flex-col gap-16 px-6 py-16">
        <header className="grid gap-10 md:grid-cols-[1.2fr_0.8fr] md:items-end">
          <div className="space-y-6">
            <p className="text-sm uppercase tracking-[0.3em] text-amber-700">
              Next.js + Tailwind
            </p>
            <h1 className="max-w-xl text-4xl font-semibold leading-tight text-slate-900 md:text-5xl font-[family:var(--font-display)]">
              A study studio for shipping ideas fast.
            </h1>
            <p className="max-w-lg text-lg leading-relaxed text-slate-600">
              Build tiny interfaces with strong composition, warm color fields, and
              purposeful typography. Each block is a small lesson that composes
              into a full layout.
            </p>
            <div className="flex flex-wrap gap-3">
              <span className="rounded-full bg-amber-100 px-4 py-2 text-xs font-semibold uppercase tracking-widest text-amber-900">
                layout rhythm
              </span>
              <span className="rounded-full bg-emerald-100 px-4 py-2 text-xs font-semibold uppercase tracking-widest text-emerald-900">
                color studies
              </span>
              <span className="rounded-full bg-slate-200 px-4 py-2 text-xs font-semibold uppercase tracking-widest text-slate-700">
                components
              </span>
            </div>
          </div>
          <div className="rounded-3xl border border-amber-200/80 bg-white/80 p-6 shadow-[0_20px_60px_-30px_rgba(15,23,42,0.6)] backdrop-blur">
            <div className="space-y-4">
              <div className="flex items-center justify-between text-sm text-slate-500">
                <span>Focus sprint</span>
                <span className="text-amber-700">Week 02</span>
              </div>
              <p className="text-2xl font-semibold text-slate-900 font-[family:var(--font-display)]">
                Layered cards, quiet shadows, and bold headlines.
              </p>
              <div className="flex items-center justify-between rounded-2xl bg-amber-50 px-4 py-3 text-sm text-slate-700">
                <span>Today</span>
                <span className="font-semibold text-slate-900">02:30 PM</span>
              </div>
            </div>
          </div>
        </header>

        <section className="grid gap-6 md:grid-cols-3">
          {[
            {
              title: "Spatial rhythm",
              detail: "Balance density with breathing room. Push the hero, pull the grid.",
              tone: "bg-white/80 border-slate-200",
            },
            {
              title: "Typography contrast",
              detail: "Pair a serif display with a crisp body to anchor hierarchy.",
              tone: "bg-amber-50/80 border-amber-200",
            },
            {
              title: "Color temperature",
              detail: "Warm highlights, cool shadows. Keep neutrals grounded.",
              tone: "bg-emerald-50/80 border-emerald-200",
            },
          ].map((card) => (
            <div
              key={card.title}
              className={`rounded-2xl border p-6 shadow-[0_18px_45px_-35px_rgba(15,23,42,0.6)] ${card.tone}`}
            >
              <h2 className="text-xl font-semibold text-slate-900 font-[family:var(--font-display)]">
                {card.title}
              </h2>
              <p className="mt-3 text-sm leading-relaxed text-slate-600">
                {card.detail}
              </p>
            </div>
          ))}
        </section>

        <section className="grid gap-8 rounded-3xl border border-slate-200/80 bg-white/70 p-8 shadow-[0_20px_60px_-30px_rgba(15,23,42,0.5)] md:grid-cols-[1.2fr_0.8fr]">
          <div className="space-y-4">
            <h3 className="text-2xl font-semibold text-slate-900 font-[family:var(--font-display)]">
              Weekly composition map
            </h3>
            <p className="text-sm leading-relaxed text-slate-600">
              Use the grid to stack small experiments. Keep each experiment
              focused on one decision: spacing, type, or motion.
            </p>
            <div className="flex flex-wrap gap-3 text-xs uppercase tracking-widest text-slate-500">
              <span className="rounded-full border border-slate-200 px-3 py-2">hero</span>
              <span className="rounded-full border border-slate-200 px-3 py-2">cards</span>
              <span className="rounded-full border border-slate-200 px-3 py-2">callout</span>
              <span className="rounded-full border border-slate-200 px-3 py-2">footer</span>
            </div>
          </div>
          <div className="space-y-4">
            <div className="rounded-2xl bg-slate-900 px-5 py-4 text-sm text-slate-100">
              <p className="text-xs uppercase tracking-widest text-amber-200">Daily focus</p>
              <p className="mt-2 text-lg font-semibold">Micro-interactions without noise.</p>
            </div>
            <div className="rounded-2xl bg-amber-100 px-5 py-4 text-sm text-amber-900">
              <p className="text-xs uppercase tracking-widest">Deliverable</p>
              <p className="mt-2 text-lg font-semibold">One refined section, no filler.</p>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
}
