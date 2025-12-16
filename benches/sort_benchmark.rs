use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sort_package_json::sort_package_json;
use std::fs;

fn bench_sort_package_json(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");

    c.bench_function("sort_package_json", |b| {
        b.iter(|| {
            let result = sort_package_json(black_box(&input));
            black_box(result)
        });
    });
}

fn bench_sort_package_json_idempotent(c: &mut Criterion) {
    let input =
        fs::read_to_string("tests/fixtures/package.json").expect("Failed to read fixture file");
    let sorted_once = sort_package_json(&input).expect("Failed to sort");

    c.bench_function("sort_package_json_idempotent", |b| {
        b.iter(|| {
            let result = sort_package_json(black_box(&sorted_once));
            black_box(result)
        });
    });
}

fn bench_sort_small_package_json(c: &mut Criterion) {
    let small_json = r#"{
        "version": "1.0.0",
        "name": "small-package",
        "dependencies": {
            "lodash": "^4.17.21"
        }
    }"#;

    c.bench_function("sort_small_package_json", |b| {
        b.iter(|| {
            let result = sort_package_json(black_box(small_json));
            black_box(result)
        });
    });
}

fn bench_sort_large_package_json(c: &mut Criterion) {
    // Create a larger, more complex package.json
    let large_json = r#"{
        "name": "large-package",
        "version": "2.5.1",
        "description": "A large package with many dependencies",
        "keywords": ["test", "benchmark", "performance", "json", "sort"],
        "homepage": "https://github.com/test/large-package",
        "bugs": {"url": "https://github.com/test/large-package/issues"},
        "repository": {"type": "git", "url": "https://github.com/test/large-package"},
        "license": "MIT",
        "author": "Test Author <author@example.com>",
        "main": "./dist/index.js",
        "module": "./dist/index.esm.js",
        "types": "./dist/index.d.ts",
        "exports": {
            ".": {
                "types": "./dist/index.d.ts",
                "import": "./dist/index.esm.js",
                "require": "./dist/index.cjs",
                "default": "./dist/index.js"
            },
            "./utils": "./dist/utils.js",
            "./helpers": "./dist/helpers.js"
        },
        "files": ["dist", "src", "README.md", "LICENSE"],
        "scripts": {
            "test": "jest",
            "build": "webpack",
            "lint": "eslint .",
            "format": "prettier --write .",
            "pretest": "npm run lint",
            "postbuild": "npm run test",
            "dev": "webpack serve",
            "clean": "rm -rf dist"
        },
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0",
            "axios": "^1.4.0",
            "lodash": "^4.17.21",
            "express": "^4.18.2",
            "moment": "^2.29.4",
            "prop-types": "^15.8.1"
        },
        "devDependencies": {
            "webpack": "^5.88.0",
            "webpack-cli": "^5.1.4",
            "jest": "^29.5.0",
            "typescript": "^5.1.3",
            "eslint": "^8.43.0",
            "prettier": "^2.8.8",
            "@types/react": "^18.2.14",
            "@types/node": "^20.3.1"
        },
        "engines": {
            "node": ">=18.0.0",
            "npm": ">=9.0.0"
        },
        "babel": {
            "presets": ["@babel/preset-env", "@babel/preset-react", "@babel/preset-typescript"],
            "plugins": ["@babel/plugin-proposal-class-properties"]
        },
        "jest": {
            "testEnvironment": "jsdom",
            "collectCoverage": true,
            "coverageDirectory": "coverage"
        },
        "eslintConfig": {
            "extends": ["eslint:recommended", "plugin:react/recommended"],
            "rules": {
                "no-console": "warn"
            }
        }
    }"#;

    c.bench_function("sort_large_package_json", |b| {
        b.iter(|| {
            let result = sort_package_json(black_box(large_json));
            black_box(result)
        });
    });
}

criterion_group!(
    benches,
    bench_sort_package_json,
    bench_sort_package_json_idempotent,
    bench_sort_small_package_json,
    bench_sort_large_package_json
);
criterion_main!(benches);
