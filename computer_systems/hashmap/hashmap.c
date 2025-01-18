#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STARTING_BUCKETS 8
#define MAX_KEY_SIZE 32

typedef struct N {
  char *key;
  void *value;
  struct N *next;
} Node;

Node *Node_new(char *key, void *value) {
  Node *n = malloc(sizeof(Node *));
  n->key = key;
  n->value = value;
  return n;
}

void Node_free(Node *n) {
  free(n->key);
  free(n);
}

void Node_set_value(Node *n, void *value) { n->value = value; }

typedef struct ULL {
  Node *head;
} UniqueLinkedList;

UniqueLinkedList *LinkedList_new() {
  UniqueLinkedList *l = malloc(sizeof(UniqueLinkedList *));
  l->head = NULL;
  return l;
}

void LinkedList_print(UniqueLinkedList *);
Node *LinkedList_get(UniqueLinkedList *, char *);

void LinkedList_push(UniqueLinkedList *ll, char *key, void *value) {
  Node *n = LinkedList_get(ll, key);
  if (n) {
    Node_set_value(n, value);
    free(key);
    return;
  }
  n = Node_new(key, value);
  n->next = ll->head;
  ll->head = n;
}

void LinkedList_delete(UniqueLinkedList *ll, char *key) {
  Node *target = ll->head;
  Node *prev_target = NULL;
  while (target) {
    // if we found the key;
    if (strcmp(key, target->key) == 0) {
      // if there is no prev target this mean the target is the head.
      // So we update the head
      if (!prev_target) {
        ll->head = target->next;
      } else {
        prev_target->next = target->next;
      }
      free(target);
      return;
    }
    prev_target = target;
    target = target->next;
  }
}

Node *LinkedList_get(UniqueLinkedList *ll, char *key) {
  Node *iter = ll->head;
  while (iter) {
    if (strcmp(key, iter->key) == 0) {
      return iter;
    }
    iter = iter->next;
  }
  return NULL;
}

void LinkedList_print(UniqueLinkedList *list) {
  if (!list || !list->head) {
    printf("Empty\n");
    return;
  }

  Node *current = list->head;
  while (current) {
    printf("(%s -> %p) -> ", current->key, current->value);
    current = current->next;
  }
  printf("NULL\n");
}

void LinkedList_free(UniqueLinkedList *ll) {
  Node *tmp;
  while (ll->head) {
    tmp = ll->head;
    ll->head = ll->head->next;
    Node_free(tmp);
  }
  free(ll);
}

typedef struct HM {
  UniqueLinkedList **arr;
} Hashmap;

size_t hash(char *s) {
  size_t sum = 0;
  size_t n = strlen(s);
  for (int i = 0; i < n; i++) {
    sum += s[i];
  }
  return sum % STARTING_BUCKETS;
}
void Hashmap_print(Hashmap *);
Hashmap *Hashmap_new() {
  Hashmap *h = malloc(sizeof(Hashmap *));
  if (!h) {
    return NULL;
  }
  h->arr = malloc(STARTING_BUCKETS * sizeof(UniqueLinkedList *));
  return h;
}

void Hashmap_set(Hashmap *h, char *key, void *value) {
  char *copy_key;
  copy_key = malloc(strlen(key));
  strcpy(copy_key, key);
  size_t hash_key = hash(copy_key);
  UniqueLinkedList *slot = h->arr[hash_key];
  // if there is already linked list in the slot;
  if (slot) {
    // Check if the key already exists.
    LinkedList_push(slot, copy_key, value);
    return;
  }

  h->arr[hash_key] = LinkedList_new();
  LinkedList_push(h->arr[hash_key], copy_key, value);
}

void *Hashmap_get(Hashmap *h, char *key) {
  UniqueLinkedList *slot = h->arr[hash(key)];
  if (slot) {
    Node *node = LinkedList_get(slot, key);
    if (node) {
      return node->value;
    }
  }
  return NULL;
}

void Hashmap_delete(Hashmap *h, char *key) {
  UniqueLinkedList *slot = h->arr[hash(key)];
  if (slot) {
    LinkedList_delete(slot, key);
  }
}

// Function to print the Hashmap
void Hashmap_print(Hashmap *hm) {
  if (!hm) {
    printf("Hashmap is NULL\n");
    return;
  }

  for (size_t i = 0; i < STARTING_BUCKETS; i++) {
    printf("Bucket %zu at %p: ", i, hm->arr[i]);
    LinkedList_print(hm->arr[i]);
  }
}

void Hashmap_free(Hashmap *h) {
  for (size_t i = 0; i < STARTING_BUCKETS; i++) {
    LinkedList_free(h->arr[i]);
  }
  free(h->arr);
  free(h);
}

int main() {
  Hashmap *h = Hashmap_new();

  // basic get/set functionality
  int a = 5;
  float b = 7.2;
  Hashmap_set(h, "item a", &a);
  Hashmap_set(h, "item b", &b);
  assert(Hashmap_get(h, "item a") == &a);
  assert(Hashmap_get(h, "item b") == &b);

  // using the same key should override the previous value
  int c = 20;
  Hashmap_set(h, "item a", &c);
  assert(Hashmap_get(h, "item a") == &c);

  // basic delete functionality
  Hashmap_delete(h, "item a");
  assert(Hashmap_get(h, "item a") == NULL);
  Hashmap_print(h);
  // handle collisions correctly
  // note: this doesn't necessarily test expansion
  int i, n = STARTING_BUCKETS * 10, ns[n];
  char key[MAX_KEY_SIZE];
  for (i = 0; i < n; i++) {
    ns[i] = i;
    sprintf(key, "item %d", i);
    Hashmap_set(h, key, &ns[i]);
    Hashmap_print(h);
  }
  Hashmap_print(h);
  for (i = 0; i < n; i++) {
    sprintf(key, "item %d", i);
    assert(Hashmap_get(h, key) == &ns[i]);
  }

  Hashmap_free(h);
  /*
     stretch goals:
     - expand the underlying array if we start to get a lot of collisions
     - support non-string keys
     - try different hash functions
     - switch from chaining to open addressing
     - use a sophisticated rehashing scheme to avoid clustered collisions
     - implement some features from Python dicts, such as reducing space use,
     maintaing key ordering etc. see https://www.youtube.com/watch?v=npw4s1QTmPg
     for ideas
     */
  printf("ok\n");
}
