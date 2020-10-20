(async () => {
    const {
        startService,
        build,
    } = require("esbuild")
    const service = await startService()

    try {
        const res = await service.build({
            entryPoints: ["./src/bind.ts"],
            outfile: './src/katex.min.js',
            format: 'esm',
            minify: true,
            bundle: true,
        })
    } finally {
        service.stop()
    }
})()
