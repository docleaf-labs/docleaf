# This code is taken from the Breathe project. The Breathe license is below.
#
# Copyright (c) 2009, Michael Jones
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without modification,
# are permitted provided that the following conditions are met:
#
#  * Redistributions of source code must retain the above copyright notice,
#    this list of conditions and the following disclaimer.
#  * Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#  * The names of its contributors may not be used to endorse or promote
#    products derived from this software without specific prior written
#    permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
# ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
# ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
# (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
# LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
# ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
# SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#

from docutils.parsers.rst.states import Text
from docutils import nodes


class NodeFinder(nodes.SparseNodeVisitor):
    """Find the Docutils desc_signature declarator and desc_content nodes."""

    def __init__(self, document):
        super().__init__(document)
        self.declarator = None
        self.signature = None
        self.content = None

    def visit_desc_signature(self, node):
        # Find the last signature node because it contains the actual declarator
        # rather than "template <...>". In Sphinx 1.4.1 we'll be able to use sphinx_cpp_tagname:
        # https://github.com/michaeljones/breathe/issues/242
        self.declarator = node
        self.signature = node

    def visit_desc_signature_line(self, node):
        # In sphinx 1.5, there is now a desc_signature_line node within the desc_signature
        # This should be used instead
        self.declarator = node

    def visit_desc_content(self, node):
        self.content = node
        # The SparseNodeVisitor seems to not actually be universally Sparse,
        # but only for nodes known to Docutils.
        # So if there are extensions with new node types in the content,
        # then the visitation will fail.
        # We anyway don't need to visit the actual content, so skip it.
        raise nodes.SkipChildren


# Taken from the Breathe code base
class InlineText(Text):
    """
    Add a custom docutils class to allow parsing inline text. This is to be
    used inside a @verbatim/@endverbatim block but only the first line is
    consumed and a inline element is generated as the parent, instead of the
    paragraph used by Text.
    """

    patterns = {"inlinetext": r""}
    initial_transitions = [("inlinetext",)]

    def indent(self, match, context, next_state):
        """
        Avoid Text's indent from detecting space prefixed text and
        doing "funny" stuff; always rely on inlinetext for parsing.
        """
        return self.inlinetext(match, context, next_state)

    def eof(self, context):
        """
        Text.eof() inserts a paragraph, so override it to skip adding elements.
        """
        return []

    def inlinetext(self, match, context, next_state):
        """
        Called by the StateMachine when an inline element is found (which is
        any text when this class is added as the single transition.
        """
        startline = self.state_machine.abs_line_number() - 1
        msg = None
        try:
            block = self.state_machine.get_text_block()
        except UnexpectedIndentationError as err:
            block, src, srcline = err.args
            msg = self.reporter.error("Unexpected indentation.", source=src, line=srcline)
        lines = context + list(block)
        text, _ = self.inline_text(lines[0], startline)
        self.parent += text
        self.parent += msg
        return [], next_state, []


# Taken from the Breathe code base
def nested_inline_parse_with_titles(state, content, node) -> str:
    """
    This code is basically a customized nested_parse_with_titles from
    docutils, using the InlineText class on the statemachine.
    """
    surrounding_title_styles = state.memo.title_styles
    surrounding_section_level = state.memo.section_level
    state.memo.title_styles = []
    state.memo.section_level = 0
    try:
        return state.nested_parse(
            content,
            0,
            node,
            match_titles=1,
            state_machine_kwargs={
                "state_classes": (InlineText,),
                "initial_state": "InlineText",
            },
        )
    finally:
        state.memo.title_styles = surrounding_title_styles
        state.memo.section_level = surrounding_section_level
