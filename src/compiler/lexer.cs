using Compiler.Token;

namespace Compiler.Lexer;

public class Lexer {
    private int left;
    private int right;
    private string source;

    public Lexer(string source) {
        left = 0;
        right = 0;
        this.source = source;
    }

    public Tokens ScanToken() {
        return Tokens.TOKEN_EOF;
    }
}