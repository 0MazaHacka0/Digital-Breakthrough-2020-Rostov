package com.smarttech.haubark.api.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

public class LoginRequest {

    @SerializedName("phone")
    private String mPhone;

    @SerializedName("password")
    private String mPassword;

    public LoginRequest(String phone, String password) {
        this.mPhone = phone;
        this.mPassword = password;
    }
}
