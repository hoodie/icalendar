# Changelog

### [v0.16.17](https://github.com/hoodie/icalendar-rs/compare/v0.16.16...v0.16.17) (2025-07-21)

#### Features

* add methods to get and set RECURRENCE-ID.
([c726cea](https://github.com/hoodie/icalendar-rs/commit/c726cea444038763543b87d81ea37b5dfe06fa2b))

### [v0.16.16](https://github.com/hoodie/icalendar-rs/compare/v0.16.15...v0.16.16) (2025-07-15)

#### Fixes

* set/access 'LAST-MODIFIED' instead of 'LAST_MODIFIED' in the corresponding
functions
([5c693e7](https://github.com/hoodie/icalendar-rs/commit/5c693e7dd0fb7c871302058a6cb3c5d9b6d0f43b))

### [v0.16.15](https://github.com/hoodie/icalendar-rs/compare/v0.16.14...v0.16.15) (2025-06-10)

#### Features

* add From-implementations for the time crate
([38a6ac2](https://github.com/hoodie/icalendar-rs/commit/38a6ac211ab7ee476b4b8a4bcf1ce2ebea6bdbee))

### [v0.16.14](https://github.com/hoodie/icalendar-rs/compare/v0.16.13...v0.16.14) (2025-06-10)

#### Features

* implement std::iter::Extend for Calendar
([bbb7090](https://github.com/hoodie/icalendar-rs/commit/bbb709001d123d8373a72308850222858d8fa417))

### [v0.16.13](https://github.com/hoodie/icalendar-rs/compare/v0.16.12...v0.16.13) (2025-02-26)

#### Fixes

* Use \n rather than \N for encoding newlines.
([c295da0](https://github.com/hoodie/icalendar-rs/commit/c295da03d6f1a3d314429330c8bed5ca7c6a9112))

### [v0.16.12](https://github.com/hoodie/icalendar-rs/compare/v0.16.11...v0.16.12) (2025-02-02)

#### Fixes

* **dep:** update to nom 8
([1edc366](https://github.com/hoodie/icalendar-rs/commit/1edc366a5efe14e2c48dc2d6046c0cce40a61bf9))

### [v0.16.11](https://github.com/hoodie/icalendar-rs/compare/v0.16.10...v0.16.11) (2025-01-14)

#### Features

* expose `DatePerhapsTime` property roundtrip methods as public API
([9972fd5](https://github.com/hoodie/icalendar-rs/commit/9972fd5796e6e6d302a1c6bb7213c5fb463525b6))
* add `get_ttl` counterpart method to `Calendar`
([e8c09f3](https://github.com/hoodie/icalendar-rs/commit/e8c09f3aa2430f3495c93244e29a026e2592fc5a))

### [v0.16.10](https://github.com/hoodie/icalendar-rs/compare/v0.16.9...v0.16.10) (2024-11-30)

#### Features

* add method for getting NaiveDate from DatePerhapsTime
([7acd8ff](https://github.com/hoodie/icalendar-rs/commit/7acd8ff26d94690927a0c23910762f6bc1f4249b)),
closes [#118](https://github.com/hoodie/icalendar-rs/issues/118)

### [v0.16.9](https://github.com/hoodie/icalendar-rs/compare/v0.16.8...v0.16.9) (2024-10-06)

#### Features

* add `LAST_MODIFIED` and `CREATED` getters and setters
([fb4021c](https://github.com/hoodie/icalendar-rs/commit/fb4021c71cb612cb6331a1d55200faa291c5b2af))

### [v0.16.8](https://github.com/hoodie/icalendar-rs/compare/v0.16.7...v0.16.8) (2024-09-10)

#### Fixes

* **deps:** update rust crate chrono-tz to 0.10
([d884935](https://github.com/hoodie/icalendar-rs/commit/d8849355776863a8ec943535539f025b01949721))

### [v0.16.7](https://github.com/hoodie/icalendar-rs/compare/v0.16.6...v0.16.7) (2024-09-09)

### [v0.16.6](https://github.com/hoodie/icalendar-rs/compare/v0.16.5...v0.16.6) (2024-09-08)

#### Fixes

* remove colon escape in text property
([70d442a](https://github.com/hoodie/icalendar-rs/commit/70d442a00f0a6db348bf5966f356ae90b76ca278))

### [v0.16.5](https://github.com/hoodie/icalendar-rs/compare/v0.16.4...v0.16.5) (2024-08-26)

#### Fixes

* deprecate pre_alloc version by making the default new generic
([4ba04cb](https://github.com/hoodie/icalendar-rs/commit/4ba04cb51b2c1c8ab85f91d72ffa3b01b0f2b467))

### [v0.16.4](https://github.com/hoodie/icalendar-rs/compare/v0.16.3...v0.16.4) (2024-08-25)

#### Fixes

* take added space into account when wrapping lines
([b7b35c2](https://github.com/hoodie/icalendar-rs/commit/b7b35c2c197d234be459dc56b6fdefdd441dc843))

### [v0.16.3](https://github.com/hoodie/icalendar-rs/compare/v0.16.2...v0.16.3) (2024-08-13)

#### Fixes

* consider value type when escaping and unescaping
([156177d](https://github.com/hoodie/icalendar-rs/commit/156177d2fde9b18a8d42a8e61a6aa74d720657d3)),
closes [#104](https://github.com/hoodie/icalendar-rs/issues/104)

### [v0.16.2](https://github.com/hoodie/icalendar-rs/compare/v0.16.1...v0.16.2) (2024-07-07)

#### Fixes

* correctly escape and unescape text
([c0ffeee](https://github.com/hoodie/icalendar-rs/commit/c0ffeee430ad12b4bd927cb44f65fc6991f9f743))
* accept quoted parameters
([38d992c](https://github.com/hoodie/icalendar-rs/commit/38d992cf75a4dfa15dc2b0cbcdc1b51b155c13c6))
* **deps:** update rust crate chrono-tz to 0.9
([37157d5](https://github.com/hoodie/icalendar-rs/commit/37157d5e2a230374fad0178562394568bfd9dfc9))

### [v0.16.1](https://github.com/hoodie/icalendar-rs/compare/v0.16.0...v0.16.1) (2024-04-07)

#### Fixes

* forbid index slicing in parser
([c0ffeec](https://github.com/hoodie/icalendar-rs/commit/c0ffeec8ad4f4875fc79e3d9cab02ab250990fdb)),
closes [#91](https://github.com/hoodie/icalendar-rs/issues/91)
* **deps:** update rust crate chrono-tz to 0.8.5
([4ef3052](https://github.com/hoodie/icalendar-rs/commit/4ef3052bc0073c9cce0a5eae4f534b3ea7fbd5fe))

## [v0.16.0](https://github.com/hoodie/icalendar-rs/compare/v0.15.8...v0.16.0) (2023-11-22)

### ⚠ BREAKING CHANGE

* `Component::multi_properties()` now returns a `&BTreeMap`


### Fixes

* parsing multi-properties
([6fe3e1f](https://github.com/hoodie/icalendar-rs/commit/6fe3e1f46007c89521e834d825d21c97f78f01d7))

### [v0.15.8](https://github.com/hoodie/icalendar-rs/compare/v0.15.7...v0.15.8) (2023-10-30)

#### Fixes

* **deps:** update rust crate chrono-tz to 0.8.4
([49d60e1](https://github.com/hoodie/icalendar-rs/commit/49d60e11e73a8d5fd214322dce6a2fbe7d50dd8f))

### [v0.15.7](https://github.com/hoodie/icalendar-rs/compare/v0.15.6...v0.15.7) (2023-08-10)

#### Fixes

* update references to renamed parser functions
([d90d950](https://github.com/hoodie/icalendar-rs/commit/d90d95005d3b3103649f34bbcf80f78f8e813585))

### [v0.15.6](https://github.com/hoodie/icalendar-rs/compare/v0.15.5...v0.15.6) (2023-07-23)

#### Fixes

* **deps:** update rust crate chrono-tz to 0.8.3
([1a31f9a](https://github.com/hoodie/icalendar-rs/commit/1a31f9a0bad05905db2e381b6e592f44cb0ee817))

### [v0.15.5](https://github.com/hoodie/icalendar-rs/compare/v0.15.4...v0.15.5) (2023-07-22)

#### Features

* add wasm support
([336dee7](https://github.com/hoodie/icalendar-rs/commit/336dee785da9a9763c854731893f6877a763a9c2))

### [v0.15.4](https://github.com/hoodie/icalendar-rs/compare/v0.15.3...v0.15.4) (2023-02-17)

#### Features

* update iso8601
([92f8329](https://github.com/hoodie/icalendar-rs/commit/92f8329c839bc9dd8d376c0f81306a524807556a))

### [v0.15.3](https://github.com/hoodie/icalendar-rs/compare/v0.15.2...v0.15.3) (2023-02-12)

#### Features

* raise MSRV to 1.60
([c434a85](https://github.com/hoodie/icalendar-rs/commit/c434a85065f901d39083c0f965cb8cd390bda0e4))
* create start and end from naivedatetime+timezone tuple
([0d9eda8](https://github.com/hoodie/icalendar-rs/commit/0d9eda8d839e646a4b12691c57fb4d6c23a1cdd8))
* add better api and examples for dates with timezones
([6245b27](https://github.com/hoodie/icalendar-rs/commit/6245b27d6d291d982cf705526d91a4dede8ebe02))

### [v0.15.2](https://github.com/hoodie/icalendar-rs/compare/v0.15.1...v0.15.2) (2023-02-01)

#### Features

* derive clone for several types
([c62d679](https://github.com/hoodie/icalendar-rs/commit/c62d679234d74c33958d3a0aafa46fcd5e0cd73f))

### [v0.15.1](https://github.com/hoodie/icalendar-rs/compare/v0.15.0...v0.15.1) (2023-01-02)

#### Fixes

* do not duplicate default properties when parsing
([d6b42d8](https://github.com/hoodie/icalendar-rs/commit/d6b42d86fe63d2e33b41672978bb34853387b418))
* add default calendar properties in From impl
([ba44631](https://github.com/hoodie/icalendar-rs/commit/ba44631fda4bacd8b104acc57de2b01eb384429a))

## [v0.15.0](https://github.com/hoodie/icalendar-rs/compare/v0.14.2...v0.15.0) (2022-12-25)

### ⚠ BREAKING CHANGE

* Calendar::default() will now contain basic properties (VERSION, CALSCALE, PRODID) which were before added during serialization.


### Features

* add basic properties to calendar in default() instead at serialization time"
([f065258](https://github.com/hoodie/icalendar-rs/commit/f0652583a73509c7fa0646588628a81bc9f92cfa))

### Fixes

* retain properties in parsed calendar
([9403dbf](https://github.com/hoodie/icalendar-rs/commit/9403dbf4c651206c3bef691f5b5b43a3bbe28d2d)),
closes [#61](https://github.com/hoodie/icalendar-rs/issues/61)

### [v0.14.2](https://github.com/hoodie/icalendar-rs/compare/v0.14.1...v0.14.2) (2022-12-15)

#### Features

* Make Calendar::property_value() publicly accessible
([ec6ae2f](https://github.com/hoodie/icalendar-rs/commit/ec6ae2f0580a01e1e27476bb3e379697de7e872d))

### [v0.14.1](https://github.com/hoodie/icalendar-rs/compare/v0.14.0...v0.14.1) (2022-12-14)

#### Fixes

* remove pretty_assertions dependency
([eb501d9](https://github.com/hoodie/icalendar-rs/commit/eb501d9e9c12bec30ca20b63169a2dbeb74cab38))

## [v0.14.0](https://github.com/hoodie/icalendar-rs/compare/v0.13.3...v0.14.0) (2022-11-20)

### ⚠ BREAKING CHANGE

* pulled certain component methods into `EventLike` trait


### Features

* add support for Alarm component
([0b04255](https://github.com/hoodie/icalendar-rs/commit/0b04255098a927bd035946deec7b505d07799356))
* add support for Alarms
([eb4a1bd](https://github.com/hoodie/icalendar-rs/commit/eb4a1bd9c7b012869e139c03fee39337e3d69930))
* add support for sequence properties
([ce57466](https://github.com/hoodie/icalendar-rs/commit/ce57466ca091490be4cff53c869b01943f901487))
* add From<Date> impl to DatePerhapsTime
([30b041a](https://github.com/hoodie/icalendar-rs/commit/30b041a0d83d60ad8653651f857d4b04f56e822c))

### [v0.13.3](https://github.com/hoodie/icalendar-rs/compare/v0.13.2...v0.13.3) (2022-10-25)

#### Fixes

* disable oldtime feature of chrono
([de14149](https://github.com/hoodie/icalendar-rs/commit/de14149f69a0666825712bceca031fdebd8a1524))

### [v0.13.2](https://github.com/hoodie/icalendar-rs/compare/v0.13.1...v0.13.2) (2022-10-18)

#### Features

* make Calendar components and properties public
([badbce2](https://github.com/hoodie/icalendar-rs/commit/badbce214c4497cf71206f2aef38021059bd12f4))
* add Component::find_prop helper
([cb10674](https://github.com/hoodie/icalendar-rs/commit/cb106742a3db54295d069de6c4985e20ba96f4f6))

### [v0.13.1](https://github.com/hoodie/icalendar-rs/compare/v0.13.0...v0.13.1) (2022-08-06)

#### Features

* convert parsed properties into DatePerhapsTime
([c0ffee9](https://github.com/hoodie/icalendar-rs/commit/c0ffee994521862f2c270aaf6b42e4c1c69556a3))

#### Fixes

* update uuid to 1.0
([c27ac42](https://github.com/hoodie/icalendar-rs/commit/c27ac4272e357ac03ef55e07cdefb9a58c58d04b))

## [v0.13.0](https://github.com/hoodie/icalendar-rs/compare/v0.12.1...v0.13.0) (2022-04-20)

### ⚠ BREAKING CHANGE

* Adds a new variant to `CalendarDateTime` enum.
* Changes the type of `Component::all_day`.
* Removes `Component::start_date` and
`Component::end_date`.
* Changes the type of `Component::starts` and
`Component::ends`.
* Changes the type of `Component::timestamp` and
`Component::get_timestamp`.


### Features

* Add support for date-time with timezone.
([e914853](https://github.com/hoodie/icalendar-rs/commit/e9148536b571171a7f8fc7fa9eacb25b3f06be39))
* Add getters for Parameter key and value.
([472f4ca](https://github.com/hoodie/icalendar-rs/commit/472f4cac70060e8321b58664f6b4ef4d93ebb97e))
* Accept Into<DatePerhapsTime> for starts and ends.
([91b1d7d](https://github.com/hoodie/icalendar-rs/commit/91b1d7dab17d0b306ac7258be9bc06e8b0a3c022))

### Fixes

* Take NaiveDate for all_day.
([193c3cf](https://github.com/hoodie/icalendar-rs/commit/193c3cfa8f9dc5efa92eccd9082f66c53a655af7))
* Remove start_date and end_date.
([c0c67fb](https://github.com/hoodie/icalendar-rs/commit/c0c67fbd4301fa04fa82fc8e1df9dc49e4190420))
* According to RFC5545, DTSTAMP must be in UTC.
([9d2fb43](https://github.com/hoodie/icalendar-rs/commit/9d2fb4330e7c72f7f253af16401d8a1ba34c2d45))

### [v0.12.1](https://github.com/hoodie/icalendar-rs/compare/v0.12.0...v0.12.1) (2022-04-15)

#### Features

* Add getter and setter for URL property.
([eb53e1a](https://github.com/hoodie/icalendar-rs/commit/eb53e1a11f84dac29381c4c06cca33fa41b2d11c))

#### Fixes

* Export DatePerhapsTime type.
([8a46f0c](https://github.com/hoodie/icalendar-rs/commit/8a46f0c4e8b33929514eb05c141595f4c23599cc))
* Fix typo in README.
([7d117b2](https://github.com/hoodie/icalendar-rs/commit/7d117b2789cc853ed396104b99a6ac7b1580a06c))

## [v0.12.0](https://github.com/hoodie/icalendar-rs/compare/v0.11.1...v0.12.0) (2022-04-13)

### Features

* Allow due date (without time) to be specified for TODO.
([2a4a451](https://github.com/hoodie/icalendar-rs/commit/2a4a451a570ebb1e0de942aae5feb4dd5564c062))
* Add getters for date-time properties.
([9619316](https://github.com/hoodie/icalendar-rs/commit/96193160328249255f9230ef118f643321cc900d))
* Add getter for ValueType on Property.
([293dc70](https://github.com/hoodie/icalendar-rs/commit/293dc70f631b62767113082dbbc9d2653f3bc50d))
* Add getters for calendar properties.
([c97acaa](https://github.com/hoodie/icalendar-rs/commit/c97acaaa366814320b207f315a7e79e4a9330f17))
* Add getter for property parameters.
([3702a20](https://github.com/hoodie/icalendar-rs/commit/3702a20fe186f5abe5422bc75ea14a90561464cf))
* Add getters for basic todo and event properties.
([dad79f4](https://github.com/hoodie/icalendar-rs/commit/dad79f4589f9cd4eaa1743ce4b55f8a89690f2c7))
* Add getters for venue properties.
([174947a](https://github.com/hoodie/icalendar-rs/commit/174947a495699e4e3146a2f7034b64df0f7bddc5))
* Add getters for basic properties.
([437770b](https://github.com/hoodie/icalendar-rs/commit/437770b2e81edee67a57bf9dec4616b7643656c2))
* derive clone and equality traits.
([5d7a8a9](https://github.com/hoodie/icalendar-rs/commit/5d7a8a9747a8700afcf697edc89307f42833c13b))

### [v0.11.1](https://github.com/hoodie/icalendar-rs/compare/v0.11.0...v0.11.1) (2022-04-03)

#### Fixes

* build docs for parser feature
([9c2db74](https://github.com/hoodie/icalendar-rs/commit/9c2db748221fdc578a4c3809c54b0325a189f35e))

## [v0.11.0](https://github.com/hoodie/icalendar-rs/compare/v0.10.0...v0.11.0) (2022-01-24)

### Features

* add timestamp setter
([c0ffee1](https://github.com/hoodie/icalendar-rs/commit/c0ffee17ec48334c55d4cdbda7a02113ac6c8e9d))
* add serde support
([c0ffee7](https://github.com/hoodie/icalendar-rs/commit/c0ffee7ea499786897b94d5c433a77698fcd45a9))
* add try_into_string to Component
([c0ffeea](https://github.com/hoodie/icalendar-rs/commit/c0ffeead7f62a7589391c15456edb9cde68ebf46))
* deprecate fmt_write
([c0ffee0](https://github.com/hoodie/icalendar-rs/commit/c0ffee0dcb0be442c2dff6415d73cbafaf264e3d))
* add TryInto<String> for &Calendar
([c0ffeed](https://github.com/hoodie/icalendar-rs/commit/c0ffeed3ea1c3a371eb97c55f5e2f5273e819908))
* add FromStr implementations
([c0ffee2](https://github.com/hoodie/icalendar-rs/commit/c0ffee25ed0c18f4ff5a1049aea3368f1ba05dc0))
* expose `CalendarElement`
([684deda](https://github.com/hoodie/icalendar-rs/commit/684dedaf3b2551ef5e74264825ddbc4e4e184489))
* add missing .done() for `Calendar`
([7bbde0e](https://github.com/hoodie/icalendar-rs/commit/7bbde0e5a6d2d7ec8ed94e91fdde5bb80736aacd))
* more From implementations for Calendar
([70c70c1](https://github.com/hoodie/icalendar-rs/commit/70c70c121512a3c193f642fb3895eea0505a401a))
* add FromStr implementation for calendar
([e670469](https://github.com/hoodie/icalendar-rs/commit/e670469242ec3c4bc6a6b26a2bad65d2264b4267))
* reserialize parsed calendar
([64c116d](https://github.com/hoodie/icalendar-rs/commit/64c116d104511542ddfaaf38101eb55bed836e9c))
* simple nom parser
([21c0cfa](https://github.com/hoodie/icalendar-rs/commit/21c0cfa2a660fc83ef91fd506c2dc5f6669d8aad))

### Fixes

* unfold by tabs as well
([c0ffee8](https://github.com/hoodie/icalendar-rs/commit/c0ffee8b480396470ea25bd2354b62bc503d320e))
* lost spaces when folding
([c0ffeea](https://github.com/hoodie/icalendar-rs/commit/c0ffeeab611d46bd71fb75ba85b755df5a8a98b8))
* conversion between parsed Property and Property lost params
([b71826d](https://github.com/hoodie/icalendar-rs/commit/b71826dcdb783e3928ff00757d63d457dbdd3910))

## [v0.10.0](https://github.com/hoodie/icalendar-rs/compare/v0.9.0...v0.10.0) (2021-02-12)

### Features

* add propery value method
([c300630](https://github.com/hoodie/icalendar-rs/commit/c300630d91718414e02d3aa3354108abcff78d53))

## [v0.9.0](https://github.com/hoodie/icalendar-rs/compare/v0.8.0...v0.9.0) (2020-02-25)

### Features

* Support calendar properties
([1c1ed6b](https://github.com/hoodie/icalendar-rs/commit/1c1ed6ba62f4f023bf47ca9ae96c0f7b3689685a))
* Implement iCalendar Venue Draft
([198911b](https://github.com/hoodie/icalendar-rs/commit/198911bb821f8c464b8a2019ea5092a097a695b6))

## [v0.8.0](https://github.com/hoodie/icalendar-rs/compare/v0.7.0...v0.8.0) (2020-01-12)

### Features

* implement FromIter for Calendar
([5e88b03](https://github.com/hoodie/icalendar-rs/commit/5e88b0350e77b38a829b84e684b752b21d6b759a))

### Fixes

* remove ToString and implement fmt::Display instead
([b52fcc4](https://github.com/hoodie/icalendar-rs/commit/b52fcc4fb81ba08faee6f4f9b479a71dca6dc853))

## [v0.7.0](https://github.com/hoodie/icalendar-rs/compare/v0.6.0...v0.7.0) (2019-11-24)

## [v0.6.0](https://github.com/hoodie/icalendar-rs/compare/v0.5.0...v0.6.0) (2019-03-09)

## [v0.5.0](https://github.com/hoodie/icalendar-rs/compare/v0.4.0...v0.5.0) (2018-12-21)

## v0.4.0 (2018-12-21)
