"""Tree data structure implementation for Python 3.7.9"""
import time


class MissingNode(Exception):
    pass


class Node(object):
    """Structure representing a node and its children."""

    type = "node"

    def __init__(self, data=None, type_=None):
        """Construct the node with data provided.

        The node's content will be set to the data provided and a type
        will be generated from it. If there is no data provided nor type
        provided, the Node will not contain neither.

        Parameters:
            data: The node's content
            type: The data type (does not need to be specified if data
                  is specified)
        """
        self.parent = None
        self.children = []
        self.root = False

        if type(data) == type:
            # Datatype entered in data, modify variables
            type_ = data
            data = None

        self.__data = data
        if type_ is None:
            self.__type = type(self.__data)
        else:
            self.__type = type_

    def add_children(self, children):
        """Add multiple child nodes.

        Parameters:
            children: An array of nodes to add
        """

        for child in children:
            if isinstance(child, Node):
                self.add_child(child)
            else:
                raise TypeError("Child is not a node")

    def add_child(self, child):
        """Add an individual child node.

        Parameters:
            child: A single Node.
        """
        if not self.has_child(child):
            child.parent = self  # Prevent recursion
            self.children.append(child)
            return child

    def remove_children(self, children):
        """Remove multiple child nodes.

        Parameters:
            children: An array of nodes to remove
        """

        for child in children:
            if isinstance(child, Node):
                self.remove_child(child)
            else:
                raise TypeError("Child is not a node")

    def remove_child(self, child):
        """Remove an individual child node.

        Parameters:
            child: A single node to remove
        """
        if self.has_child(child):
            child.set_parent(None)
            self.children.remove(child)
        else:
            raise ValueError("Child argument is not a child of this node")

    def set_parent(self, parent):
        """Set the current parent node.

        Parameters:
            parent: The parent node to set to.
        """
        self.parent = parent

        # Add self as parent's child
        # If self is already a child, it'll be ignored
        parent.add_child(self)

    def set_root(self, tree):
        """Set the node as root of a Tree.

        Parameters:
            tree: The Tree to set the current node the root node in
        """
        if tree.type == "tree":
            self.root = True
            self.parent = tree
            tree.root_node = self
        else:
            raise TypeError("root is not of type Tree")

    def set_data(self, data):
        """Modify the node's data.

        Parameters:
            data: Set the node's data to this
        """
        if isinstance(data, self.__type) or data is None:
            self.__data = data
        else:
            raise TypeError(
                f"Data '{type(data)}' is not of node's type '{self.__type.__name__}'"
            )

    def is_top(self):
        """Check if the node is either a root node or has no parent."""
        return self.root or self.parent is None

    def has_child(self, comparative):
        """Check if it has a child node or any children that satisfy a lambda function."""
        if type(comparative) == Node:
            return comparative in self.children

        # Lambda function
        return len([i for i in self.children if comparative(i)]) > 0

    def get_children(self, condition=None):
        """Get all children of the node that, optionally, satisfy the provided condition.

        Return all children which -- if a condition if specified -- satisfies the
        lambda condition.

        Parameters:
            condition: Lambda condition to check each children with
        """
        if condition is None:
            return self.children

        return [child for child in self.children if condition(child)]

    def get_parent(self):
        """Get the parent of the node."""
        return self.parent

    def get_layer(self):
        """Get the node layer.

        Layer 0 is root, layer 1 is 0's children and so on.
        """
        return self.__recursion_root(self, count=(True, 0))

    def get_data(self):
        """Return the node's data."""
        return self.__data

    def get_type(self):
        """Return the node's datatype."""
        return self.__type

    def get_root(self):
        """Traverse and find the root node."""

        return self.__recursion_root(self)

    def __recursion_root(self, node, count=(False, 0)):
        """Recursively climb up parents from a starting node and get the root node.

        Using this function you can either find the root node or the layers
        between the root node and the starting node. It finds the parent node
        of the current node and repeats the function until there is either
        no parent (for root nodes without a tree) or it is self-defined as a
        root node (for root nodes in a tree).

        Parameters:
            node: The current/starting node
            count: The return value

        Return value:
            If counting,
                it returns the layers between the root node and starting node
            else,
                return the root node.
        """
        if (node.root) or (node.parent is None):
            return count[1] if count[0] else node
        else:
            if count[0]:
                newcount = (True, count[1] + 1)
            else:
                newcount = count
            return self.__recursion_root(node.parent, newcount)


class Tree(object):
    """The tree data structure representing a collection of nodes.

    Parameters:
        root_node: The parent node of the tree

    Attributes:

    """

    type = "tree"

    def __init__(self, root_node=None):
        self.set_root(root_node)

    def set_root(self, root_node):
        """Set the root node of the tree.

        Parameters
        ----------
        root_node : Node
            The node to set as root.
        """
        if isinstance(root_node, Node):
            root_node.set_root(self)
        elif root_node is not None:
            if root_node is None:
                # Remove the root node
                if self.root_node is not None:
                    self.root_node.parent = None

                self.root_node = None
            else:
                raise TypeError("Root node is not a node")

    def find_node(self, condition):
        """Use the Depth First Search algorithm to find a node that satisfies a lambda condition.

        The DFS algorithm will start at the root node and search depth first rather than
        breadth. For every unexplored node it encounters, it will check if its content
        satisfies a certain condition.
        """

        if self.root_node:
            # Root node exists
            rv = self.__depth_first_search(self.root_node, condition)
            return rv
        else:
            raise MissingNode("Root node is not specified for Tree")

    def __depth_first_search(self, node, condition):
        """The Depth First Search Algorithm

        Parameters
        ----------
        node : Node
            The current node
        condition : function
            The condition to find a certain node

        Returns
        -------
        Node/None
            The target node which satisfies `condition`

        """
        if node:
            if not node.is_top():
                if condition(node):
                    return node

            for child in node.children:
                rv = self.__depth_first_search(child, condition)
                if type(rv) is Node:
                    return rv
