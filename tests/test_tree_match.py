import unittest

from lxml import etree
from mdr._treelib import tree_size, _simple_tree_match # from ._tree
from mdr.tree import clustered_tree_match

s1 = """<root>
            <b>
                <d></d>
                <e></e>
            </b>
            <c>
                <f></f>
            </c>
            <b>
                <e></e>
                <d></d>
            </b>
            <c>
                <g>
                    <h></h>
                    <i></i>
                    <j></j>
                </g>
            </c>
        </root>
"""  # tree size = 14

s2 = """<root>
            <b>
                <d></d>
                <e></e>
            </b>
            <c>
                <g>
                    <h></h>
                </g>
                <f></f>
            </c>
        </root>
"""  # tree size = 8

class TreeMatchTest(unittest.TestCase):

    def test_tree_match(self):
        tree1 = etree.XML(s1)
        tree2 = etree.XML(s2)
        self.assertEquals(14, tree_size(tree1))
        self.assertEquals(8, tree_size(tree2))
        self.assertEquals(7, _simple_tree_match(tree1, tree2))
        self.assertEquals(0.375, clustered_tree_match(tree1, tree2))
