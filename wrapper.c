#include "c_eval/poker.h"
#include "c_eval/pokerlib.cpp"

int eval_7cards(int c0, int c1, int c2, int c3, int c4, int c5, int c6)
{
    int cards[7] = {c0, c1, c2, c3, c4, c5, c6};
    return eval_7hand(cards);
}