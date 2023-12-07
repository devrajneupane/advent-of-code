from collections import Counter

res = 0
tmp = {}
cards = ("A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J")
hands_bid = {
    items[0]: int(items[1])
    for items in (line.split() for line in open(0).read().splitlines())
}

# categorize hands based on hand type
for hand in hands_bid:
    cnt = Counter(hand).most_common()
    cnt_len, max_cnt = len(cnt), cnt[0][1]

    # J card can pretend to be whatever card is best for the purpose of determining hand type
    if "J" in hand and max_cnt < 5:
        d = dict(cnt)
        j_cnt = d["J"]
        if j_cnt >= max_cnt:
            max_cnt = cnt[1][1]
        max_cnt += j_cnt
        cnt_len -= 1

    if 1 < cnt_len < 4 and max_cnt == 3:
        cnt_len += 1
    elif cnt_len > 2 and max_cnt < 3:
        cnt_len += 2

    tmp.setdefault(cnt_len, []).append(hand)

# sort hands in each category based on cards
for grp, hands in tmp.items():
    tmp[grp] = sorted(hands, key=lambda x: [-cards.index(char) for char in x])

# sort category based on hands type and extract hands
tmp = [b for a in sorted(tmp.items(), reverse=True) for b in a[1]]

# determine total winnings
for rank, hand in enumerate(tmp, start=1):
    res += rank * hands_bid[hand]

print(res)
