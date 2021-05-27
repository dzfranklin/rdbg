package view

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import org.danielzfranklin.rdbg.CodeViewOuterClass.*

@Composable
fun CodeView(source: String, highlights: List<HlRange>?) {
    val text = AnnotatedString.Builder(source).apply {
        if (highlights != null) {
            for (hl in highlights) {
                addStyle(getStyle(hl.highlight), hl.range.start, hl.range.end)
            }
        }
    }.toAnnotatedString()

    Text(text)
}

fun getStyle(hl: Highlight): SpanStyle {
    TODO()
}
