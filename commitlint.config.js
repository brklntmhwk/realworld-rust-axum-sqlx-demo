module.exports = {
    extends: ["@commitlint/config-conventional"],
    parserPreset: {
        parserOpts: {
            headerPattern: /^(\w*)\((\w*)\):\s(.*)$/,
            headerCorrespondence: ["type", "scope", "subject"]
        }
    },
    rules: {
        "references-empty": [2, "never"],
        "scope-empty": [1, "never"],
    },
    helpUrl:
        "https://github.com/conventional-changelog/commitlint/#what-is-commitlint",
};