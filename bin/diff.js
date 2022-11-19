#!/usr/bin/env node

const SUCCESS_REGEX = /(?<=change:\n)(.+\n)(.+\n)(\s+)(?=Performance has improved)/g
const FAILURE_REGEX = /(?<=change:\n)(.+\n)(.+\n)(\s+)(?=Performance has regressed)/g

process.stdin.on('readable', () => {
    const data = process.stdin.read();

    if (data) {
        const output = data.toString()
            .replaceAll(SUCCESS_REGEX, "+$1+$2+$3")
            .replaceAll(FAILURE_REGEX, "-$1-$2-$3");

        console.log(output);
    }
});
