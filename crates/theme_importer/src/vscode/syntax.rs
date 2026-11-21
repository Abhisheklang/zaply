use indexmap::IndexMap;
use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColor {
    pub name: Option<String>,
    pub scope: Option<VsCodeTokenScope>,
    pub settings: VsCodeTokenColorSettings,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColorSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum ZaplySyntaxToken {
    Attribute,
    Boolean,
    Comment,
    CommentDoc,
    Constant,
    Constructor,
    Embedded,
    Emphasis,
    EmphasisStrong,
    Enum,
    Function,
    Hint,
    Keyword,
    Label,
    LinkText,
    LinkUri,
    Number,
    Operator,
    Predictive,
    Preproc,
    Primary,
    Property,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationListMarker,
    PunctuationSpecial,
    String,
    StringEscape,
    StringRegex,
    StringSpecial,
    StringSpecialSymbol,
    Tag,
    TextLiteral,
    Title,
    Type,
    Variable,
    VariableSpecial,
    Variant,
}

impl std::fmt::Display for ZaplySyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ZaplySyntaxToken::Attribute => "attribute",
                ZaplySyntaxToken::Boolean => "boolean",
                ZaplySyntaxToken::Comment => "comment",
                ZaplySyntaxToken::CommentDoc => "comment.doc",
                ZaplySyntaxToken::Constant => "constant",
                ZaplySyntaxToken::Constructor => "constructor",
                ZaplySyntaxToken::Embedded => "embedded",
                ZaplySyntaxToken::Emphasis => "emphasis",
                ZaplySyntaxToken::EmphasisStrong => "emphasis.strong",
                ZaplySyntaxToken::Enum => "enum",
                ZaplySyntaxToken::Function => "function",
                ZaplySyntaxToken::Hint => "hint",
                ZaplySyntaxToken::Keyword => "keyword",
                ZaplySyntaxToken::Label => "label",
                ZaplySyntaxToken::LinkText => "link_text",
                ZaplySyntaxToken::LinkUri => "link_uri",
                ZaplySyntaxToken::Number => "number",
                ZaplySyntaxToken::Operator => "operator",
                ZaplySyntaxToken::Predictive => "predictive",
                ZaplySyntaxToken::Preproc => "preproc",
                ZaplySyntaxToken::Primary => "primary",
                ZaplySyntaxToken::Property => "property",
                ZaplySyntaxToken::Punctuation => "punctuation",
                ZaplySyntaxToken::PunctuationBracket => "punctuation.bracket",
                ZaplySyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                ZaplySyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                ZaplySyntaxToken::PunctuationSpecial => "punctuation.special",
                ZaplySyntaxToken::String => "string",
                ZaplySyntaxToken::StringEscape => "string.escape",
                ZaplySyntaxToken::StringRegex => "string.regex",
                ZaplySyntaxToken::StringSpecial => "string.special",
                ZaplySyntaxToken::StringSpecialSymbol => "string.special.symbol",
                ZaplySyntaxToken::Tag => "tag",
                ZaplySyntaxToken::TextLiteral => "text.literal",
                ZaplySyntaxToken::Title => "title",
                ZaplySyntaxToken::Type => "type",
                ZaplySyntaxToken::Variable => "variable",
                ZaplySyntaxToken::VariableSpecial => "variable.special",
                ZaplySyntaxToken::Variant => "variant",
            }
        )
    }
}

impl ZaplySyntaxToken {
    pub fn find_best_token_color_match<'a>(
        &self,
        token_colors: &'a [VsCodeTokenColor],
    ) -> Option<&'a VsCodeTokenColor> {
        let mut ranked_matches = IndexMap::new();

        for (ix, token_color) in token_colors.iter().enumerate() {
            if token_color.settings.foreground.is_none() {
                continue;
            }

            let Some(rank) = self.rank_match(token_color) else {
                continue;
            };

            if rank > 0 {
                ranked_matches.insert(ix, rank);
            }
        }

        ranked_matches
            .into_iter()
            .max_by_key(|(_, rank)| *rank)
            .map(|(ix, _)| &token_colors[ix])
    }

    fn rank_match(&self, token_color: &VsCodeTokenColor) -> Option<u32> {
        let candidate_scopes = match token_color.scope.as_ref()? {
            VsCodeTokenScope::One(scope) => vec![scope],
            VsCodeTokenScope::Many(scopes) => scopes.iter().collect(),
        }
        .iter()
        .flat_map(|scope| scope.split(',').map(|s| s.trim()))
        .collect::<Vec<_>>();

        let scopes_to_match = self.to_vscode();
        let number_of_scopes_to_match = scopes_to_match.len();

        let mut matches = 0;

        for (ix, scope) in scopes_to_match.into_iter().enumerate() {
            // Assign each entry a weight that is inversely proportional to its
            // position in the list.
            //
            // Entries towards the front are weighted higher than those towards the end.
            let weight = (number_of_scopes_to_match - ix) as u32;

            if candidate_scopes.contains(&scope) {
                matches += 1 + weight;
            }
        }

        Some(matches)
    }

    pub fn fallbacks(&self) -> &[Self] {
        match self {
            ZaplySyntaxToken::CommentDoc => &[ZaplySyntaxToken::Comment],
            ZaplySyntaxToken::Number => &[ZaplySyntaxToken::Constant],
            ZaplySyntaxToken::VariableSpecial => &[ZaplySyntaxToken::Variable],
            ZaplySyntaxToken::PunctuationBracket
            | ZaplySyntaxToken::PunctuationDelimiter
            | ZaplySyntaxToken::PunctuationListMarker
            | ZaplySyntaxToken::PunctuationSpecial => &[ZaplySyntaxToken::Punctuation],
            ZaplySyntaxToken::StringEscape
            | ZaplySyntaxToken::StringRegex
            | ZaplySyntaxToken::StringSpecial
            | ZaplySyntaxToken::StringSpecialSymbol => &[ZaplySyntaxToken::String],
            _ => &[],
        }
    }

    fn to_vscode(self) -> Vec<&'static str> {
        match self {
            ZaplySyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            ZaplySyntaxToken::Boolean => vec!["constant.language"],
            ZaplySyntaxToken::Comment => vec!["comment"],
            ZaplySyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            ZaplySyntaxToken::Constant => vec!["constant", "constant.language", "constant.character"],
            ZaplySyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
            }
            ZaplySyntaxToken::Embedded => vec!["meta.embedded"],
            ZaplySyntaxToken::Emphasis => vec!["markup.italic"],
            ZaplySyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            ZaplySyntaxToken::Enum => vec!["support.type.enum"],
            ZaplySyntaxToken::Function => vec![
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            ZaplySyntaxToken::Hint => vec![],
            ZaplySyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
            ZaplySyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            ZaplySyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            ZaplySyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            ZaplySyntaxToken::Number => vec!["constant.numeric", "number"],
            ZaplySyntaxToken::Operator => vec!["operator", "keyword.operator"],
            ZaplySyntaxToken::Predictive => vec![],
            ZaplySyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            ZaplySyntaxToken::Primary => vec![],
            ZaplySyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            ZaplySyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.definition.tag",
            ],
            ZaplySyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            ZaplySyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            ZaplySyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            ZaplySyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            ZaplySyntaxToken::String => vec!["string"],
            ZaplySyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            ZaplySyntaxToken::StringRegex => vec!["string.regex"],
            ZaplySyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            ZaplySyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            ZaplySyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            ZaplySyntaxToken::TextLiteral => vec!["text.literal", "string"],
            ZaplySyntaxToken::Title => vec!["title", "entity.name"],
            ZaplySyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            ZaplySyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            ZaplySyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            ZaplySyntaxToken::Variant => vec!["variant"],
        }
    }
}
