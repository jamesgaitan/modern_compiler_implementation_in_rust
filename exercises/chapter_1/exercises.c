#include "util.h"
#include "stddef.h"

typedef struct tree *T_tree;
struct tree
{
    T_tree left;
    string key;
    T_tree right;
};

T_tree Tree(T_tree l, string k, T_tree r)
{
    T_tree t = checked_malloc(sizeof(*t));
    t->left = l;
    t->key = k;
    t->right = r;
    return t;
}

T_tree insert(string key, T_tree t)
{
    if (t == NULL)
        return Tree(NULL, key, NULL);
    else if (strcmp(key, t->key) < 0)
        return Tree(insert(key, t->left), t->key, t->right);
    else if (strcmp(key, t->key) > 0)
        return Tree(t->left, t->key, insert(key, t->right));
    else
        return Tree(t->left, key, t->right);
}

bool member(string key, T_tree t)
{
    if (t == NULL)
    {
        return false;
    }

    if (strncmp(key, t->key, MAX_STRING_LEN) == 0)
    {
        return true;
    }
    else
    {
        return member(key, t->left) || member(key, t->right);
    }

    return false;
}

int main()
{
    T_tree tree = Tree(Tree(NULL, "world", NULL), "hello", NULL);

    printf("member(hi) = %d\n", member("hi", tree));
    printf("member(world) = %d\n", member("world", tree));
}