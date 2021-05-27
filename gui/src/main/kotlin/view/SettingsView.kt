package view

import Repo
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier

@Composable
fun SettingsView(repo: Repo) {
    val categoriesScrollState = rememberScrollState()

    val categories = listOf(Category("Theme", ::ThemeView))
    val selectedCategory = remember { mutableStateOf(0) }

    Row {
        Column(Modifier.selectableGroup().verticalScroll(categoriesScrollState)) {
            for ((idx, cat) in categories.withIndex()) {
                Text(cat.name, Modifier.selectable(idx == selectedCategory.value) { selectedCategory.value = idx })
            }
        }

    }
}

@Composable
fun ThemeView(repo: Repo) {}

data class Category(
    val name: String,
    val view: @Composable (Repo) -> Unit,
    var selected: MutableState<Boolean>,
)
