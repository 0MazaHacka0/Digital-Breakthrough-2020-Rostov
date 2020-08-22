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

        votes[voteID] = Vote(_home_id, _description);
    }

    function newUser(uint _user_id, uint _home_id) public returns (uint userID) {
        require(msg.sender == owner);
        userID = countUsers++;

        users[userID] = User(_user_id, _home_id);
    }
}