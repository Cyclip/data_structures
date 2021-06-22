from ds_tree import Tree, Node
import time
import json
import random

dictionary = json.load(open("dictionary.json", "r"))
dictionary.sort()

tree = Tree()
root = Node()


def add_word(word):
    def add_letter(node, word, charIndex):
        checkLetterFunc = lambda x: word[charIndex] == x.get_data()
        if not node.has_child(checkLetterFunc):
            # If the next letter is not a child, add it
            addedNode = node.add_child(Node(word[charIndex]))
        else:
            # Next letter is a child, get it
            addedNode = node.get_children(checkLetterFunc)[0]

        if charIndex + 1 > len(word) - 1:
            # Reached the end of the word, add the word itself
            finalNode = addedNode.add_child(Node(word))
            return finalNode

        add_letter(addedNode, word, charIndex + 1)

    add_letter(root, word.lower(), 0)


def add_words():
    start = time.time()
    for word in dictionary:
        add_word(word)
    end = time.time()
    print(f"Added {len(dictionary)} words in {round(end-start, 4)}s")


add_words()  # Add words to the tree
tree.set_root(root)  # Set root node for searching
targetVal = random.choice(dictionary)  # The target value to find


"""
    Searching via Depth First Search algorithm
    This isn't ideal for this tree but it serves as a
    test.
    (Ideally it should check each children of a node and
    select the best matching one to build the name).

    Speed: O(|V| + |E|)
        V: Vertices
        E: Edges
"""

print(f"\nSearching DFS for {targetVal}..")
start = time.time()
result = tree.find_node(lambda x: x.get_data() == targetVal)
end = time.time()

print(f"Search for {result.get_data()} took {end-start}s (layer {result.get_layer()})")

"""
    A more efficient search for this specific use case.
    It searches only the necessary nodes so is approximately
    2x faster.
"""


def search(tree, word):
    global nodes

    def _search(node, word, index):
        if index == len(word):
            # Finished
            finalWord = node.get_children(lambda x: x.get_data() == word)
            return None if len(finalWord) == 0 else finalWord[0]

        expectedLetter = word[index]
        nextLetterNode = node.get_children(lambda x: x.get_data() == expectedLetter)

        if nextLetterNode:
            return _search(nextLetterNode[0], word, index + 1)
        else:
            return None

    start = time.time()
    result = _search(tree.root_node, word, 0)
    end = time.time()

    print(f"Search for {word} took {end-start}s")

    if result:
        print(f"Successful (layer {result.get_layer()})")
    else:
        print("Failed.")


print(f"\nSearching for {targetVal}..")
search(tree, targetVal)
