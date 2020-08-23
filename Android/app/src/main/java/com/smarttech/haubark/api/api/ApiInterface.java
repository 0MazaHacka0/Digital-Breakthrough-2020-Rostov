package com.smarttech.haubark.api.api;

import com.smarttech.haubark.api.model.LoginRequest;

import retrofit2.Call;
import retrofit2.http.Body;
import retrofit2.http.Field;
import retrofit2.http.FormUrlEncoded;
import retrofit2.http.Header;
import retrofit2.http.POST;

public interface ApiInterface {

    // Login
    @POST("login")
    Call<Void> login(@Body LoginRequest loginRequest);

//    @FormUrlEncoded
//    @POST("managerLogin")
//    Call<ManagerLoginResponse> managerLogin(@Field("username") String username, @Field("password") String password, @Field("grant_type") String grant_type);

}
