# Resources

## Crates 
* https://crates.io/crates/evmap - concurrent map focuesd on fast reads, can be used for handling currency
* https://crates.io/crates/fluent - Translation and localization
* https://crates.io/crates/uom - SI units, can be used for weight and length based units
* https://crates.io/crates/num-rational - Could be used for price
* https://crates.io/crates/rust_decimal - Could be used for price
* https://crates.io/crates/fluent - Handle l10n
* https://crates.io/crates/oso - Could be used as access rules and maybe building discount engine
* https://crates.io/crates/tracing - Used to log and debug
* https://crates.io/crates/secrecy - Could be used to hide secrets from logs and api

## Third party data
* https://openexchangerates.org/ - api for currency, has a free tier with 1000 requests per month
* http://cldr.unicode.org/index/downloads - localization data for all countries and languages
* http://www.unece.org/cefact/locode/welcome.html - Regions for countries
* https://www.geonames.org/ - Translated names for country regions

## Other things
* https://www.openpolicyagent.org/ - could be used for access rules and discount engine.
  Could use the library or build a simple engine around the policy language (rego)
