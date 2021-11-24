#pragma once

#include <memory>
#include <string>

enum class Suit : int {
    SPADES = 1,
    DIAMONDS = 2,
    CLUBS = 3,
    HEARTS = 4,
};

Suit nameToSuit(const std::string& name);
std::unique_ptr<std::string> suitToName(Suit suit);
