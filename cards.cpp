#include "cards.hpp"

#include <unordered_map>

static const std::unordered_map<Suit, std::string> kSuitNameMap = {
    {Suit::SPADES, "spades"},
    {Suit::DIAMONDS, "diamonds"},
    {Suit::CLUBS, "clubs"},
    {Suit::HEARTS, "hearts"},
};

static const std::unordered_map<std::string, Suit> kNameSuitMap = {
    {"spades", Suit::SPADES},
    {"diamonds", Suit::DIAMONDS},
    {"clubs", Suit::CLUBS},
    {"hearts", Suit::HEARTS},
};

Suit nameToSuit(const std::string& name) { return kNameSuitMap.at(name); }

std::unique_ptr<std::string> suitToName(Suit suit) {
    return std::make_unique<std::string>(kSuitNameMap.at(suit));
}
