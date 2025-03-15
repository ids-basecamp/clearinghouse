# [2.0.0-beta.2](https://github.com/ids-basecamp/clearinghouse/compare/v2.0.0-beta.1...v2.0.0-beta.2) (2025-03-15)


### Bug Fixes

* **app:** Bump dependencies ([3c34b8a](https://github.com/ids-basecamp/clearinghouse/commit/3c34b8adf906cfae0da1e167bed024d0be509d39))
* **app:** Multiple small changes ([39d0d7d](https://github.com/ids-basecamp/clearinghouse/commit/39d0d7da4f9897cc1e176216aa67cca12f86c698))
* **app:** Possible race condition when log and create_process are executed at the same time; Formatting ([024812d](https://github.com/ids-basecamp/clearinghouse/commit/024812df05587c4cb220a7b322f8cd08069edcc7))
* clearing house edc docker build ([ae6a9c4](https://github.com/ids-basecamp/clearinghouse/commit/ae6a9c46e5e1e4fd60a37ddf35cd5ff4a8ca6e25))
* **edc:** docker build ([936754b](https://github.com/ids-basecamp/clearinghouse/commit/936754bb00a6f811815abe1d07f808edecea502c))
* fixing test workflow ([7fe976f](https://github.com/ids-basecamp/clearinghouse/commit/7fe976ffd0312b295a18fbbcdf59904390694185))
* fixing test workflow ([c04c22f](https://github.com/ids-basecamp/clearinghouse/commit/c04c22ff9abc5aa8a943cfb4dd3a90e505fe393e))
* fixing test workflow ([c195e2c](https://github.com/ids-basecamp/clearinghouse/commit/c195e2cdfaeb3288a5d263b3417de975152803fd))
* fixing unit tests ([5cc791e](https://github.com/ids-basecamp/clearinghouse/commit/5cc791ea9a17a688e26552db81cdf0172d7eb7a3))
* upgrading edc fork version ([0219b6a](https://github.com/ids-basecamp/clearinghouse/commit/0219b6a604f88c1cab94b72d36da668e5be46158))


* Merge pull request #126 from ids-basecamp/feature/lh-2.0 ([7efec2c](https://github.com/ids-basecamp/clearinghouse/commit/7efec2c725b1d8ff148d98966b9a5c8fb2e68ebb)), closes [#126](https://github.com/ids-basecamp/clearinghouse/issues/126)


### Features

* **app:** add 'static_process_owner' for processes ([#117](https://github.com/ids-basecamp/clearinghouse/issues/117)) ([33cd21a](https://github.com/ids-basecamp/clearinghouse/commit/33cd21a3f3fdb5d7ac14d612ca1f18b0af0d9831))
* **app:** removed MongoDB implementation, added report posting in PR, bumped dependencies ([c7a6aeb](https://github.com/ids-basecamp/clearinghouse/commit/c7a6aeb39987b7e621d0878c5c3b5b955c5e17e2))
* clearing house app response errors mapping ([32dc0d4](https://github.com/ids-basecamp/clearinghouse/commit/32dc0d44d181e816d274129bd3844077734e0698))
* clearing house app response errors mapping ([c9fa461](https://github.com/ids-basecamp/clearinghouse/commit/c9fa4610a5d9d71476445aa4db2ff9387716a138))
* Delete infomodel and edc classes to use classes from forked projects ([8571093](https://github.com/ids-basecamp/clearinghouse/commit/857109395a21ad6b3152421abe66e8747d521537))
* **docs:** setup deployment docs ([#124](https://github.com/ids-basecamp/clearinghouse/issues/124)) ([6a67af1](https://github.com/ids-basecamp/clearinghouse/commit/6a67af137243f6ed1cb75add12f1f75ab234c46d))
* Included pagging tests for the query endpoint ([177cbee](https://github.com/ids-basecamp/clearinghouse/commit/177cbeed72f37f706e808b0fe1b722427311c285))
* removing duplicated infomodel and edc classes ([0c3b7d6](https://github.com/ids-basecamp/clearinghouse/commit/0c3b7d6780ebd800b7f312cb21a0dc580e9279f7))
* removing duplicated infomodel and edc classes ([946550e](https://github.com/ids-basecamp/clearinghouse/commit/946550e8849e220c3e4f3241884825c3ff3b0568))
* Update project to use edc-fork classes ([53364d4](https://github.com/ids-basecamp/clearinghouse/commit/53364d4f484b177dac3914c471c8b8c08c4e9ff1))
* updates to run tests ([0e25281](https://github.com/ids-basecamp/clearinghouse/commit/0e252812c2764bd4b063d1936fba2396118c8f74))
* updates to run tests ([9fd4abd](https://github.com/ids-basecamp/clearinghouse/commit/9fd4abd13c45fe4429149082618e1e708f27955e))
* updating edc version ([c246d4e](https://github.com/ids-basecamp/clearinghouse/commit/c246d4eb6d6d4baae31490c5655c91c1b92bd7fa))
* updating to use forked edc and gradle-plugins ([75bc25c](https://github.com/ids-basecamp/clearinghouse/commit/75bc25c826ef91fb7a147093e328d03d6d963556))


### BREAKING CHANGES

* implement ids protocol directly in app and remove EDC

# [2.0.0-beta.1](https://github.com/ids-basecamp/clearinghouse/compare/v1.0.0...v2.0.0-beta.1) (2025-02-13)


* 2_0_beta.1 (#130) ([297c288](https://github.com/ids-basecamp/clearinghouse/commit/297c28820fda002b1f9412ab642eb8eab0bc435f)), closes [#130](https://github.com/ids-basecamp/clearinghouse/issues/130)


### BREAKING CHANGES

* edc as a proxy connector has been removed and the ids prodoctol directly implemented in the app

# 1.0.0 (2024-09-30)


### Bug Fixes

* **app:** Fix build on development branch ([32bfea3](https://github.com/ids-basecamp/clearinghouse/commit/32bfea389a3f0f43907f3c5e7afa66105f25cf60))
* **app:** Fix build on development branch ([851146e](https://github.com/ids-basecamp/clearinghouse/commit/851146eb3c546f6813d3209beee367b84ee1ffaa))
* **app:** Fix warnings and build on development branch ([ef8bf76](https://github.com/ids-basecamp/clearinghouse/commit/ef8bf76e772b0b23076f6e5a633281ecc12a6e9e))
* **app:** Fix warnings and build on development branch ([89f39f7](https://github.com/ids-basecamp/clearinghouse/commit/89f39f784180b4bd26813f33e7787d0744fe975c))
* **ch-app:** Add error log and removed assert ([0d07fe5](https://github.com/ids-basecamp/clearinghouse/commit/0d07fe55c3a83a2b4d22adde2e7c70ddc44b2c06))
* **ch-app:** Bump dependencies ([6f273bb](https://github.com/ids-basecamp/clearinghouse/commit/6f273bbd5b8c0503f2061aee944b95c692a2a3f1))
* **ch-app:** copy migrations in Dockerfile ([b91926c](https://github.com/ids-basecamp/clearinghouse/commit/b91926cd6dbde60e1e13813949587d3a6f3e3f4c))
* **ch-app:** Fix 3 vulnerabilitites: GHSA-rjhf-4mh8-9xjq, GHSA-xphf-cx8h-7q9g, GHSA-3mv5-343c-w2qg ([2ca4dfa](https://github.com/ids-basecamp/clearinghouse/commit/2ca4dfae59aa65061f818d579d81eb7f09325576))
* **ch-app:** Fix all clippy warnings ([812f3e8](https://github.com/ids-basecamp/clearinghouse/commit/812f3e868bfb4c17c5a18765bacaf7826ef99532))
* **ch-app:** Fix integration test case log ([bcc6a56](https://github.com/ids-basecamp/clearinghouse/commit/bcc6a5604162d6d4166f00e57587e9bab049c565))
* **ch-app:** Fix security issue through updating dependencies ([2613559](https://github.com/ids-basecamp/clearinghouse/commit/26135597ccc4a8f9f040f496732fb7e275504ce9))
* **ch-app:** Fixed uuid <-> str mismatch in document, which resulted in failed query ([0571bd1](https://github.com/ids-basecamp/clearinghouse/commit/0571bd1d720d89d9c3b9d3758d70197faca4f04c))
* **ch-app:** Reenable new serde crates, due to resolved issues with precompiled binaries ([e2784b9](https://github.com/ids-basecamp/clearinghouse/commit/e2784b9b642987cc1ddb9ffa2ca7057cb6382d25))
* **ch-app:** Updated dependencies to fix security vulnerability ([fe19cdf](https://github.com/ids-basecamp/clearinghouse/commit/fe19cdf8c153a1108759a27f689ed3fdc2197ff4))
* **ch-edc:** add missing vault filesystem ([e845269](https://github.com/ids-basecamp/clearinghouse/commit/e845269a2149f9b02b5dac71c4f40649052a8d12))
* **ch-edc:** add multistage dockerfile ([8e8026e](https://github.com/ids-basecamp/clearinghouse/commit/8e8026e39059debc5df27f24b58829c081c58da0))
* change tests for `referringConnector` test ([25cd379](https://github.com/ids-basecamp/clearinghouse/commit/25cd379c969c8747f618fc5166e97957631504e8))
* changed repository in package.json ([5b0b15c](https://github.com/ids-basecamp/clearinghouse/commit/5b0b15cdf5f44ffe6e38b556c6573d19a9ffce7e))
* **ci:** change docker image tag to reflect new repo and impl job matrix ([60379b4](https://github.com/ids-basecamp/clearinghouse/commit/60379b464c8e00591555462cce1d4820619b274f))
* **ci:** Delete .github/workflows/rust.yml to fix failing CI ([3a8d5a1](https://github.com/ids-basecamp/clearinghouse/commit/3a8d5a15c08151ea2d43f70d7a25ecb4f4555424))
* **ci:** disable rust workflow (dublicate build) ([9af75cf](https://github.com/ids-basecamp/clearinghouse/commit/9af75cf760173fda5d1fad4bf4ddbefd21224413))
* **ci:** Fix rust.yml workflow ([0a474c0](https://github.com/ids-basecamp/clearinghouse/commit/0a474c0904a74f258978b1bd0ed2278edd8c8db1))
* **ci:** Fix unauthorized push ([57d4e02](https://github.com/ids-basecamp/clearinghouse/commit/57d4e02ebee80c04f359d577fd87af2a70e0b7ce))
* **ci:** Fix unauthorized push ([453ce88](https://github.com/ids-basecamp/clearinghouse/commit/453ce8810ddd5970f0d7c349f142ea5f24db8b8a))
* **ci:** simplified ch-edc docker build ([f0cb1e1](https://github.com/ids-basecamp/clearinghouse/commit/f0cb1e149160b945e6e03d2426e6b40165c6fb55))
* **ci:** updated test job to run from root ([04cecce](https://github.com/ids-basecamp/clearinghouse/commit/04cecce30c0c787847ca199788d40e1daf07092f))
* **config:** Fixed config and added unit test to verify correct functionality ([76765e6](https://github.com/ids-basecamp/clearinghouse/commit/76765e687c3cac025f33fd902d28a6caec764e2f))
* **core:** Disable integration tests, fix warnings and make the build reproducible ([ecd3078](https://github.com/ids-basecamp/clearinghouse/commit/ecd3078b92d8061588f58537133c5b56074b91f9))
* **core:** Disable integration tests, fix warnings and make the build reproducible ([c69b246](https://github.com/ids-basecamp/clearinghouse/commit/c69b246cf365c06ccfb23bdf0c85f0506f4a023e))
* disable tokenFormat check ([c920b82](https://github.com/ids-basecamp/clearinghouse/commit/c920b825219edeae317d874f6cb723d1016ecabc))
* GITHUB_TOKEN permissions for release job ([b2678aa](https://github.com/ids-basecamp/clearinghouse/commit/b2678aaa49bb9d2d0259413567704b7670635bc1))
* quick start docker-compose.yml snytax ([0d83989](https://github.com/ids-basecamp/clearinghouse/commit/0d8398932fb4fde1b454d2117ef567cc85ddc0c0))
* removed workingdir since cd is used ([34e2b9a](https://github.com/ids-basecamp/clearinghouse/commit/34e2b9ad64c1e95e969450c412745412b852d716))
* **tests:** add __ENV for hostname and token ([209244c](https://github.com/ids-basecamp/clearinghouse/commit/209244c551e8e9fd4eed5e00b620a271e5fd57e9))
* updating .gitignore to exclude vscode files ([1ce073f](https://github.com/ids-basecamp/clearinghouse/commit/1ce073fef0b2e70d97c58d1b14a7dec104bed3a1))


### Features

* AppSender, LoggingMessageDelegate, LogMessageHandler tests implemented ([5127591](https://github.com/ids-basecamp/clearinghouse/commit/5127591162bec3ee6e92227ffbb80f36ffa08f62))
* basic endpoint functions working ([f1726e7](https://github.com/ids-basecamp/clearinghouse/commit/f1726e74574a596e1216d4cf468af1ccfd07443e))
* **ch-app:** Add and debug integration test ([cef068b](https://github.com/ids-basecamp/clearinghouse/commit/cef068b2e41916a05101dab5e3255114a49a95c8))
* **ch-app:** Add create_process test and fix an issue ([8cfb5e1](https://github.com/ids-basecamp/clearinghouse/commit/8cfb5e18feea759aeb4425cb900453f86f07c15f))
* **ch-app:** Add CreateProcessResponse as JSON ([002845a](https://github.com/ids-basecamp/clearinghouse/commit/002845aa0729887853954118032084c6e5606354))
* **ch-app:** add Dockerfile and GH action ([f64aa14](https://github.com/ids-basecamp/clearinghouse/commit/f64aa14c802e91a34b85437d07d79eba756ea504))
* **ch-app:** Add docs for installation of ch-app ([293500d](https://github.com/ids-basecamp/clearinghouse/commit/293500d45f2bccbae47d4ae0dfdbf01851ea4f03))
* **ch-app:** Add postgres implementation ([#96](https://github.com/ids-basecamp/clearinghouse/issues/96)) ([842ff00](https://github.com/ids-basecamp/clearinghouse/commit/842ff0058b0b6d1ca4b3d62a6747d0bfcf025bb8))
* **ch-app:** Add testcontainers for Integration tests with database ([679b06b](https://github.com/ids-basecamp/clearinghouse/commit/679b06b95d8e7ac58019fd21e678c6725c79083e))
* **ch-app:** Added tests, refactored unwrap ([b3f8ede](https://github.com/ids-basecamp/clearinghouse/commit/b3f8edec027aa8168f64fd552ec7bed0e7f4ac30))
* **ch-app:** Bump Cargo edition to 2021 and remove unused imports ([007281f](https://github.com/ids-basecamp/clearinghouse/commit/007281f3e7f436606c04c41edab917c432e7e0c8))
* **ch-app:** Bump Cargo edition to 2021 and remove unused imports ([6a3934e](https://github.com/ids-basecamp/clearinghouse/commit/6a3934e089f775bf434821d0e672e63daf34676c))
* **ch-app:** Created services for Keyring- and Document-Service inside logging service and adjusted the handlers ([4bb512f](https://github.com/ids-basecamp/clearinghouse/commit/4bb512f68f1137a3c89cca7bbd4ee6055525b1ed))
* **ch-app:** Created services for Keyring- and Document-Service inside logging service and adjusted the handlers ([f1a8e59](https://github.com/ids-basecamp/clearinghouse/commit/f1a8e5969006156c931ce39a7225b8e3acea56a5))
* **ch-app:** feature flag sentry ([918a903](https://github.com/ids-basecamp/clearinghouse/commit/918a9035ac1e61a0faa8716143f25886d049dae2))
* **ch-app:** Finished error-handling in keyring service and introduces 'doc_type' feature ([387498c](https://github.com/ids-basecamp/clearinghouse/commit/387498c15ff2bd8c2890625dd92d8d3be1250b42))
* **ch-app:** Finished refactoring document-service error-handling ([8965f5e](https://github.com/ids-basecamp/clearinghouse/commit/8965f5e8a1ccbfdf8c36040f3736a3dd7fee7929))
* **ch-app:** Implement [#91](https://github.com/ids-basecamp/clearinghouse/issues/91) ([965b4c2](https://github.com/ids-basecamp/clearinghouse/commit/965b4c2cbba0580006f9e40834470f3e225354b6))
* **ch-app:** Refactor logging-api to use a service as well ([4259c65](https://github.com/ids-basecamp/clearinghouse/commit/4259c65cfca978f3ad77c8d37fec85bd3fbaa90f))
* **ch-app:** Refactor logging-api to use a service as well ([f1beee0](https://github.com/ids-basecamp/clearinghouse/commit/f1beee0bd6ed48277d02a385b25d232f7ee5740a))
* **ch-app:** Remove Blockchain, add integration tests ([ffdfbad](https://github.com/ids-basecamp/clearinghouse/commit/ffdfbadd10769b99f392617f0d691fcd45dcdafb))
* **ch-app:** Removed ApiResponse, fixed warnings and hid more doc_type related functions ([fc710b7](https://github.com/ids-basecamp/clearinghouse/commit/fc710b7afc2f8ff28729ee88315fd74777476c05))
* **ch-app:** Removed certs folder ([2779f6c](https://github.com/ids-basecamp/clearinghouse/commit/2779f6c5fc2f550e9e35af9c60b2ca7426d52036))
* **ch-app:** Setup tracing as logger and replace rocket as logger; setup config ([c9d8e6f](https://github.com/ids-basecamp/clearinghouse/commit/c9d8e6f99fba95ab83816911293cc1885f866fae))
* **ch-app:** Setup tracing as logger and replace rocket as logger; setup config ([356665a](https://github.com/ids-basecamp/clearinghouse/commit/356665a46bd6de165b0fd227b845d10d6e1fcb0e))
* **ch-app:** Use JWKS from endpoint to validate receipt ([11a7314](https://github.com/ids-basecamp/clearinghouse/commit/11a7314f2bfc9236561770623a98239bf71b088e))
* **ci:** add test job for CH app ([807bcdf](https://github.com/ids-basecamp/clearinghouse/commit/807bcdf5fad95456dfcd008fcee990983facd711))
* create connector and extension modules ([fa47ff8](https://github.com/ids-basecamp/clearinghouse/commit/fa47ff8f18feeefd77fdcf6be9cfe266981f358b))
* Create TestUtils with mock and start to create application tests ([f1612e0](https://github.com/ids-basecamp/clearinghouse/commit/f1612e027f9815ad9525c7f78aab876baf1f64a1))
* **doc:** Add internal description to docs ([4e89ba6](https://github.com/ids-basecamp/clearinghouse/commit/4e89ba6755095d30d23df8caec3463561112cafe))
* **docker:** Optimised docker image with distroless image ([d046826](https://github.com/ids-basecamp/clearinghouse/commit/d046826132c1e6cc3e60f2c31e2d4f8c397fe01b))
* **docs:** add d2 diagramming integration to workflow ([24e87ef](https://github.com/ids-basecamp/clearinghouse/commit/24e87efc96516a22dc1edc4d89662cebd537d2bf))
* **docs:** add mdbook for documentation ([0cf4ada](https://github.com/ids-basecamp/clearinghouse/commit/0cf4adaa5494a8ae3bc679ee0387b90bc3079e38))
* **docs:** Enable GitHub Pages generation ([36bfaa3](https://github.com/ids-basecamp/clearinghouse/commit/36bfaa3f569ee86be8f8cc072cb951aeaca8e295))
* enable pedantic linter and fix clippy findings where appropiate ([df0a5d4](https://github.com/ids-basecamp/clearinghouse/commit/df0a5d40ed50ea45f90383c21666b51fb89bdddd))
* externalization of environments variables ([f8e187e](https://github.com/ids-basecamp/clearinghouse/commit/f8e187e59c32483c8250252683804f0b86643de7))
* readme added ([4d382b5](https://github.com/ids-basecamp/clearinghouse/commit/4d382b5877dda24b6143b08a47549d3c29a61d71))
* release action ([98f1448](https://github.com/ids-basecamp/clearinghouse/commit/98f1448795003bf6fc823fccda7f0e14fe8b7cb0))
* release action ([4710fc0](https://github.com/ids-basecamp/clearinghouse/commit/4710fc0bde1a63ca6af2042a56b81b68c73860b1))
* **release:** add more release types ([cd59461](https://github.com/ids-basecamp/clearinghouse/commit/cd59461fb2dfa5b8c95c80fbaa3bafd511e036c0))
* semantic-release ([6fb29ff](https://github.com/ids-basecamp/clearinghouse/commit/6fb29ff39a86a34e2bda5ac400b1114643b4f906))
* starting create objects and method ([f13f15e](https://github.com/ids-basecamp/clearinghouse/commit/f13f15e7e35c866f011a4474bc3bd5722d8a40b9))
* **tests:** add load tests ([a88175b](https://github.com/ids-basecamp/clearinghouse/commit/a88175bb083ce0091459e8b47c4c27ac042f782b))
* **tests:** add smoke tests ([e31f806](https://github.com/ids-basecamp/clearinghouse/commit/e31f8066b08ebac341aa3b081056bbd110b72680))
* uses now `referringConnector` instead of `SKI:AKI` ([b472344](https://github.com/ids-basecamp/clearinghouse/commit/b472344d7bb9e9f63dc4c97bbf3545e7d761d8f6))

# [1.0.0-beta.3](https://github.com/ids-basecamp/clearinghouse/compare/v1.0.0-beta.2...v1.0.0-beta.3) (2024-03-15)


### Bug Fixes

* change tests for `referringConnector` test ([25cd379](https://github.com/ids-basecamp/clearinghouse/commit/25cd379c969c8747f618fc5166e97957631504e8))


### Features

* **ch-app:** Add create_process test and fix an issue ([8cfb5e1](https://github.com/ids-basecamp/clearinghouse/commit/8cfb5e18feea759aeb4425cb900453f86f07c15f))
* **ch-app:** Add testcontainers for Integration tests with database ([679b06b](https://github.com/ids-basecamp/clearinghouse/commit/679b06b95d8e7ac58019fd21e678c6725c79083e))
* enable pedantic linter and fix clippy findings where appropiate ([df0a5d4](https://github.com/ids-basecamp/clearinghouse/commit/df0a5d40ed50ea45f90383c21666b51fb89bdddd))
* uses now `referringConnector` instead of `SKI:AKI` ([b472344](https://github.com/ids-basecamp/clearinghouse/commit/b472344d7bb9e9f63dc4c97bbf3545e7d761d8f6))

# [1.0.0-beta.2](https://github.com/ids-basecamp/clearinghouse/compare/v1.0.0-beta.1...v1.0.0-beta.2) (2024-02-19)


### Bug Fixes

* **ch-app:** copy migrations in Dockerfile ([b91926c](https://github.com/ids-basecamp/clearinghouse/commit/b91926cd6dbde60e1e13813949587d3a6f3e3f4c))
* **ch-app:** Fix 3 vulnerabilitites: GHSA-rjhf-4mh8-9xjq, GHSA-xphf-cx8h-7q9g, GHSA-3mv5-343c-w2qg ([2ca4dfa](https://github.com/ids-basecamp/clearinghouse/commit/2ca4dfae59aa65061f818d579d81eb7f09325576))
* **ch-app:** Fixed uuid <-> str mismatch in document, which resulted in failed query ([0571bd1](https://github.com/ids-basecamp/clearinghouse/commit/0571bd1d720d89d9c3b9d3758d70197faca4f04c))
* changed repository in package.json ([5b0b15c](https://github.com/ids-basecamp/clearinghouse/commit/5b0b15cdf5f44ffe6e38b556c6573d19a9ffce7e))
* **ci:** change docker image tag to reflect new repo and impl job matrix ([60379b4](https://github.com/ids-basecamp/clearinghouse/commit/60379b464c8e00591555462cce1d4820619b274f))
* disable tokenFormat check ([c920b82](https://github.com/ids-basecamp/clearinghouse/commit/c920b825219edeae317d874f6cb723d1016ecabc))
* GITHUB_TOKEN permissions for release job ([b2678aa](https://github.com/ids-basecamp/clearinghouse/commit/b2678aaa49bb9d2d0259413567704b7670635bc1))


### Features

* **ch-app:** Add postgres implementation ([#96](https://github.com/ids-basecamp/clearinghouse/issues/96)) ([842ff00](https://github.com/ids-basecamp/clearinghouse/commit/842ff0058b0b6d1ca4b3d62a6747d0bfcf025bb8))
* **ch-app:** Implement [#91](https://github.com/ids-basecamp/clearinghouse/issues/91) ([965b4c2](https://github.com/ids-basecamp/clearinghouse/commit/965b4c2cbba0580006f9e40834470f3e225354b6))

# 1.0.0-beta.1 (2023-11-23)


### Bug Fixes

* **app:** Fix build on development branch ([32bfea3](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/32bfea389a3f0f43907f3c5e7afa66105f25cf60))
* **app:** Fix build on development branch ([851146e](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/851146eb3c546f6813d3209beee367b84ee1ffaa))
* **app:** Fix warnings and build on development branch ([89f39f7](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/89f39f784180b4bd26813f33e7787d0744fe975c))
* **ch-app:** Add error log and removed assert ([0d07fe5](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/0d07fe55c3a83a2b4d22adde2e7c70ddc44b2c06))
* **ch-app:** Bump dependencies ([6f273bb](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/6f273bbd5b8c0503f2061aee944b95c692a2a3f1))
* **ch-app:** Fix all clippy warnings ([812f3e8](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/812f3e868bfb4c17c5a18765bacaf7826ef99532))
* **ch-app:** Fix integration test case log ([bcc6a56](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/bcc6a5604162d6d4166f00e57587e9bab049c565))
* **ch-app:** Fix security issue through updating dependencies ([2613559](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/26135597ccc4a8f9f040f496732fb7e275504ce9))
* **ch-app:** Reenable new serde crates, due to resolved issues with precompiled binaries ([e2784b9](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/e2784b9b642987cc1ddb9ffa2ca7057cb6382d25))
* **ch-app:** Updated dependencies to fix security vulnerability ([fe19cdf](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/fe19cdf8c153a1108759a27f689ed3fdc2197ff4))
* **ch-edc:** add missing vault filesystem ([e845269](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/e845269a2149f9b02b5dac71c4f40649052a8d12))
* **ch-edc:** add multistage dockerfile ([8e8026e](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/8e8026e39059debc5df27f24b58829c081c58da0))
* **ci:** Delete .github/workflows/rust.yml to fix failing CI ([3a8d5a1](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/3a8d5a15c08151ea2d43f70d7a25ecb4f4555424))
* **ci:** disable rust workflow (dublicate build) ([9af75cf](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/9af75cf760173fda5d1fad4bf4ddbefd21224413))
* **ci:** Fix rust.yml workflow ([0a474c0](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/0a474c0904a74f258978b1bd0ed2278edd8c8db1))
* **ci:** Fix unauthorized push ([57d4e02](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/57d4e02ebee80c04f359d577fd87af2a70e0b7ce))
* **ci:** Fix unauthorized push ([453ce88](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/453ce8810ddd5970f0d7c349f142ea5f24db8b8a))
* **ci:** simplified ch-edc docker build ([f0cb1e1](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f0cb1e149160b945e6e03d2426e6b40165c6fb55))
* **ci:** updated test job to run from root ([04cecce](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/04cecce30c0c787847ca199788d40e1daf07092f))
* **config:** Fixed config and added unit test to verify correct functionality ([76765e6](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/76765e687c3cac025f33fd902d28a6caec764e2f))
* **core:** Disable integration tests, fix warnings and make the build reproducible ([ecd3078](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/ecd3078b92d8061588f58537133c5b56074b91f9))
* **core:** Disable integration tests, fix warnings and make the build reproducible ([c69b246](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/c69b246cf365c06ccfb23bdf0c85f0506f4a023e))
* quick start docker-compose.yml snytax ([0d83989](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/0d8398932fb4fde1b454d2117ef567cc85ddc0c0))
* removed workingdir since cd is used ([34e2b9a](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/34e2b9ad64c1e95e969450c412745412b852d716))
* **tests:** add __ENV for hostname and token ([209244c](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/209244c551e8e9fd4eed5e00b620a271e5fd57e9))
* updating .gitignore to exclude vscode files ([1ce073f](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/1ce073fef0b2e70d97c58d1b14a7dec104bed3a1))


### Features

* AppSender, LoggingMessageDelegate, LogMessageHandler tests implemented ([5127591](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/5127591162bec3ee6e92227ffbb80f36ffa08f62))
* basic endpoint functions working ([f1726e7](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f1726e74574a596e1216d4cf468af1ccfd07443e))
* **ch-app:** Add and debug integration test ([cef068b](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/cef068b2e41916a05101dab5e3255114a49a95c8))
* **ch-app:** Add CreateProcessResponse as JSON ([002845a](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/002845aa0729887853954118032084c6e5606354))
* **ch-app:** add Dockerfile and GH action ([f64aa14](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f64aa14c802e91a34b85437d07d79eba756ea504))
* **ch-app:** Add docs for installation of ch-app ([293500d](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/293500d45f2bccbae47d4ae0dfdbf01851ea4f03))
* **ch-app:** Added tests, refactored unwrap ([b3f8ede](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/b3f8edec027aa8168f64fd552ec7bed0e7f4ac30))
* **ch-app:** Bump Cargo edition to 2021 and remove unused imports ([007281f](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/007281f3e7f436606c04c41edab917c432e7e0c8))
* **ch-app:** Bump Cargo edition to 2021 and remove unused imports ([6a3934e](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/6a3934e089f775bf434821d0e672e63daf34676c))
* **ch-app:** Created services for Keyring- and Document-Service inside logging service and adjusted the handlers ([4bb512f](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/4bb512f68f1137a3c89cca7bbd4ee6055525b1ed))
* **ch-app:** Created services for Keyring- and Document-Service inside logging service and adjusted the handlers ([f1a8e59](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f1a8e5969006156c931ce39a7225b8e3acea56a5))
* **ch-app:** feature flag sentry ([918a903](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/918a9035ac1e61a0faa8716143f25886d049dae2))
* **ch-app:** Finished error-handling in keyring service and introduces 'doc_type' feature ([387498c](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/387498c15ff2bd8c2890625dd92d8d3be1250b42))
* **ch-app:** Finished refactoring document-service error-handling ([8965f5e](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/8965f5e8a1ccbfdf8c36040f3736a3dd7fee7929))
* **ch-app:** Refactor logging-api to use a service as well ([4259c65](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/4259c65cfca978f3ad77c8d37fec85bd3fbaa90f))
* **ch-app:** Refactor logging-api to use a service as well ([f1beee0](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f1beee0bd6ed48277d02a385b25d232f7ee5740a))
* **ch-app:** Remove Blockchain, add integration tests ([ffdfbad](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/ffdfbadd10769b99f392617f0d691fcd45dcdafb))
* **ch-app:** Removed ApiResponse, fixed warnings and hid more doc_type related functions ([fc710b7](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/fc710b7afc2f8ff28729ee88315fd74777476c05))
* **ch-app:** Removed certs folder ([2779f6c](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/2779f6c5fc2f550e9e35af9c60b2ca7426d52036))
* **ch-app:** Setup tracing as logger and replace rocket as logger; setup config ([c9d8e6f](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/c9d8e6f99fba95ab83816911293cc1885f866fae))
* **ch-app:** Setup tracing as logger and replace rocket as logger; setup config ([356665a](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/356665a46bd6de165b0fd227b845d10d6e1fcb0e))
* **ch-app:** Use JWKS from endpoint to validate receipt ([11a7314](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/11a7314f2bfc9236561770623a98239bf71b088e))
* **ci:** add test job for CH app ([807bcdf](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/807bcdf5fad95456dfcd008fcee990983facd711))
* create connector and extension modules ([fa47ff8](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/fa47ff8f18feeefd77fdcf6be9cfe266981f358b))
* Create TestUtils with mock and start to create application tests ([f1612e0](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f1612e027f9815ad9525c7f78aab876baf1f64a1))
* **doc:** Add internal description to docs ([4e89ba6](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/4e89ba6755095d30d23df8caec3463561112cafe))
* **docker:** Optimised docker image with distroless image ([d046826](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/d046826132c1e6cc3e60f2c31e2d4f8c397fe01b))
* **docs:** add d2 diagramming integration to workflow ([24e87ef](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/24e87efc96516a22dc1edc4d89662cebd537d2bf))
* **docs:** add mdbook for documentation ([0cf4ada](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/0cf4adaa5494a8ae3bc679ee0387b90bc3079e38))
* **docs:** Enable GitHub Pages generation ([36bfaa3](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/36bfaa3f569ee86be8f8cc072cb951aeaca8e295))
* externalization of environments variables ([f8e187e](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f8e187e59c32483c8250252683804f0b86643de7))
* readme added ([4d382b5](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/4d382b5877dda24b6143b08a47549d3c29a61d71))
* release action ([98f1448](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/98f1448795003bf6fc823fccda7f0e14fe8b7cb0))
* release action ([4710fc0](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/4710fc0bde1a63ca6af2042a56b81b68c73860b1))
* **release:** add more release types ([cd59461](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/cd59461fb2dfa5b8c95c80fbaa3bafd511e036c0))
* semantic-release ([6fb29ff](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/6fb29ff39a86a34e2bda5ac400b1114643b4f906))
* starting create objects and method ([f13f15e](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/f13f15e7e35c866f011a4474bc3bd5722d8a40b9))
* **tests:** add load tests ([a88175b](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/a88175bb083ce0091459e8b47c4c27ac042f782b))
* **tests:** add smoke tests ([e31f806](https://github.com/truzzt/ids-basecamp-clearinghouse/commit/e31f8066b08ebac341aa3b081056bbd110b72680))
