use super::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageCode {
    code: String,
    wire: &'static str,
}

impl LanguageCode {
    pub fn code(&self) -> &str {
        &self.code
    }

    pub(crate) fn wire(&self) -> &'static str {
        self.wire
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum SourceLang {
    #[default]
    Auto,
    Known(LanguageCode),
}

impl SourceLang {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let normalized = normalize_input(input);
        if normalized.is_empty() || normalized == "AUTO" {
            return Ok(Self::Auto);
        }

        source_wire_code(&normalized)
            .map(|wire| {
                Self::Known(LanguageCode {
                    code: normalized,
                    wire,
                })
            })
            .ok_or_else(|| Error::UnsupportedSourceLang(input.to_string()))
    }

    pub(crate) fn wire(&self) -> Option<&'static str> {
        match self {
            Self::Auto => None,
            Self::Known(code) => Some(code.wire()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetLang(LanguageCode);

impl TargetLang {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let normalized = normalize_input(input);
        if normalized.is_empty() {
            return Err(Error::MissingTargetLang);
        }
        if normalized == "AUTO" {
            return Err(Error::AutoTargetLang);
        }

        target_wire_code(&normalized)
            .map(|wire| {
                Self(LanguageCode {
                    code: normalized,
                    wire,
                })
            })
            .ok_or_else(|| Error::UnsupportedTargetLang(input.to_string()))
    }

    pub fn code(&self) -> &str {
        self.0.code()
    }

    pub(crate) fn wire(&self) -> &'static str {
        self.0.wire()
    }
}

fn normalize_input(input: &str) -> String {
    input.trim().replace('_', "-").to_ascii_uppercase()
}

fn source_wire_code(code: &str) -> Option<&'static str> {
    match code {
        "EN" => Some("en"),
        "PT" => Some("pt"),
        _ => target_wire_code(code),
    }
}

fn target_wire_code(code: &str) -> Option<&'static str> {
    match code {
        "AR" => Some("ar"),
        "BG" => Some("bg"),
        "CS" => Some("cs"),
        "DA" => Some("da"),
        "DE" => Some("de"),
        "EL" => Some("el"),
        "EN-GB" => Some("en-GB"),
        "EN-US" => Some("en-US"),
        "ES" => Some("es"),
        "ES-419" => Some("es-419"),
        "ET" => Some("et"),
        "FI" => Some("fi"),
        "FR" => Some("fr"),
        "HE" => Some("he"),
        "HU" => Some("hu"),
        "ID" => Some("id"),
        "IT" => Some("it"),
        "JA" => Some("ja"),
        "KO" => Some("ko"),
        "LT" => Some("lt"),
        "LV" => Some("lv"),
        "NB" => Some("nb"),
        "NL" => Some("nl"),
        "PL" => Some("pl"),
        "PT-BR" => Some("pt-BR"),
        "PT-PT" => Some("pt-PT"),
        "RO" => Some("ro"),
        "RU" => Some("ru"),
        "SK" => Some("sk"),
        "SL" => Some("sl"),
        "SV" => Some("sv"),
        "TR" => Some("tr"),
        "UK" => Some("uk"),
        "VI" => Some("vi"),
        "ZH" => Some("zh-Hans"),
        "ZH-HANS" => Some("zh-Hans"),
        "ZH-HANT" => Some("zh-Hant"),
        // Convenience aliases for legacy callers
        "EN" => Some("en-US"),
        "PT" => Some("pt-BR"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_auto_source() {
        assert_eq!(SourceLang::parse("auto").unwrap(), SourceLang::Auto);
        assert_eq!(SourceLang::parse("").unwrap(), SourceLang::Auto);
    }

    #[test]
    fn rejects_auto_target() {
        assert!(matches!(
            TargetLang::parse("auto"),
            Err(Error::AutoTargetLang)
        ));
    }

    #[test]
    fn maps_target_aliases_to_regional_defaults() {
        assert_eq!(TargetLang::parse("EN").unwrap().wire(), "en-US");
        assert_eq!(TargetLang::parse("PT").unwrap().wire(), "pt-BR");
    }

    #[test]
    fn maps_source_aliases_to_generic_codes() {
        assert_eq!(SourceLang::parse("EN").unwrap().wire(), Some("en"));
        assert_eq!(SourceLang::parse("PT").unwrap().wire(), Some("pt"));
    }

    #[test]
    fn rejects_unsupported_languages() {
        assert!(matches!(
            SourceLang::parse("xx"),
            Err(Error::UnsupportedSourceLang(_))
        ));
        assert!(matches!(
            TargetLang::parse("xx"),
            Err(Error::UnsupportedTargetLang(_))
        ));
    }
}
