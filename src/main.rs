mod runbook;
use tower_lsp::LspService;
use tower_lsp::Server;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }
    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.client
            .log_message(MessageType::INFO, format!("file opened: {}, {}", uri, text))
            .await;
        self.validate_and_publish_diagnostics(uri, text).await;
    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("file changed: {}, {}", uri, text),
                )
                .await;
            self.validate_and_publish_diagnostics(uri, text).await;
        }
    }
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        self.client
            .log_message(MessageType::INFO, format!("file saved: {}", uri))
            .await;
    }
}
impl Backend {
    async fn validate_and_publish_diagnostics(&self, _: Url, text: String) {
        use crate::runbook::parse_yaml;
        let mut diags = vec![];
        match parse_yaml(&text) {
            Ok(_) => {
                diags.push(Diagnostic {
                    range: Range::default(),
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: None,
                    code_description: None,
                    source: Some("runn-yaml-ls".into()),
                    message: "No steps defined in runbook".into(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
            Err(e) => {
                let msg = format!("YAML parse error: {}", e);
                // 行・列番号がわかれば Range に入れる
                let diag = Diagnostic {
                    range: Range {
                        start: Position {
                            line: e
                                .location()
                                .map_or(0, |mark| mark.line().try_into().unwrap_or(0)),
                            character: e
                                .location()
                                .map_or(0, |mark| mark.column().try_into().unwrap_or(0)),
                        },
                        end: Position {
                            line: 0,
                            character: 1,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("runn-yaml-ls".into()),
                    message: msg,
                    related_information: None,
                    tags: None,
                    data: None,
                };
                diags.push(diag);
            }
        }
    }
}
#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // スキーマを出力する例
    let (service, socket) = LspService::build(|client| Backend { client }).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
