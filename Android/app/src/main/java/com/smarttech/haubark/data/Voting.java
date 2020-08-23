package com.smarttech.haubark.data;

public class Voting {

    private String mTitle;

    private int mVoteYes;
    private int mVoteNo;
    private int mVoteAbstained;

    public Voting(String title, int yes, int no, int abstained) {
        this.mTitle = title;
        this.mVoteYes = yes;
        this.mVoteNo = no;
        this.mVoteAbstained = abstained;
    }

    public String getTitle() {
        return mTitle;
    }

    public int getVoteYes() {
        return mVoteYes;
    }

    public int getVoteNo() {
        return mVoteNo;
    }

    public int getVoteAbstained() {
        return mVoteAbstained;
    }
}
