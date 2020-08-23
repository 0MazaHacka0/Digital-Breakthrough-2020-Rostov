pragma solidity >=0.4.25 <0.7.0;

contract System_Votes {
    address public owner;
    enum State { Agree, Disagree, Abstained } 

    struct User {
        uint user_id;
        uint home_id;
    }

    struct Voice {
        uint user_id;
        State state;
    }

    struct Vote {
        uint home_id;
        string description;
        uint count_voices;
        mapping (uint => Voice) voices;
    }

    mapping (uint => Vote) votes;
    mapping (uint => User) users;

    uint countVotes;
    uint countUsers;

    constructor() public {
        owner = msg.sender;
    }

    function createVote(uint _home_id, string memory _description) public returns (uint voteID) {
        require(msg.sender == owner);
        voteID = countVotes++;
        votes[voteID] = Vote(_home_id, _description, 0);

        return voteID;
    }

    function newUser(uint _user_id, uint _home_id) public returns (uint userID) {
        require(msg.sender == owner);
        userID = countUsers++;

        users[userID] = User(_user_id, _home_id);
    }

    function addVoice(uint _vote_id, uint _user_id, State state) public returns (uint voiceID) {
        require(msg.sender == owner);

        voiceID = votes[_vote_id].count_voices + 1;
        votes[_vote_id].count_voices++;
        votes[_vote_id].voices[voiceID] = Voice(_user_id, state);

        return voiceID;
    }

    function getVotesIds(uint _home_id) public view returns (uint[] memory) {
        require(msg.sender == owner);

        uint count = 0;
        uint[] memory votesIds = new uint[](countVotes);

        for (uint i = 0; i < countVotes; i++) {
            if(votes[i].home_id == _home_id) {
                votesIds[count] = i;
                count++;
            }
        }

        return votesIds;
    }

    function getVote(uint _vote_id) public view returns (uint, string memory, State[] memory) {
        State[] memory states = new State[](votes[_vote_id].count_voices);

        for (uint i = 0; i < votes[_vote_id].count_voices; i++) {
            states[i] = votes[_vote_id].voices[i].state;
        }

        return (votes[_vote_id].home_id, votes[_vote_id].description, states);
    }
}