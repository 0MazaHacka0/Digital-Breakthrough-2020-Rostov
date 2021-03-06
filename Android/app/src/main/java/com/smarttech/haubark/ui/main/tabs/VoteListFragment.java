package com.smarttech.haubark.ui.main.tabs;

import android.os.Bundle;

import androidx.fragment.app.Fragment;

import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.ListView;

import com.smarttech.haubark.R;
import com.smarttech.haubark.data.Voting;
import com.smarttech.haubark.ui.main.tabs.vote_list.VoteAdapter;

import java.util.ArrayList;
import java.util.List;

/**
 * A simple {@link Fragment} subclass.
 * Use the {@link VoteListFragment#newInstance} factory method to
 * create an instance of this fragment.
 */
public class VoteListFragment extends Fragment {

    // TODO: Rename parameter arguments, choose names that match
    // the fragment initialization parameters, e.g. ARG_ITEM_NUMBER
    private static final String ARG_PARAM1 = "param1";
    private static final String ARG_PARAM2 = "param2";

    // TODO: Rename and change types of parameters
    private String mParam1;
    private String mParam2;

    public VoteListFragment() {
        // Required empty public constructor
    }

    /**
     * Use this factory method to create a new instance of
     * this fragment using the provided parameters.
     *
     * @return A new instance of fragment VoteListFragment.
     */
    // TODO: Rename and change types and number of parameters
    public static VoteListFragment newInstance() {
        VoteListFragment fragment = new VoteListFragment();
        Bundle args = new Bundle();
        fragment.setArguments(args);
        return fragment;
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        if (getArguments() != null) {
            mParam1 = getArguments().getString(ARG_PARAM1);
            mParam2 = getArguments().getString(ARG_PARAM2);
        }

    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState) {

        // Inflate the layout for this fragment
        View view = inflater.inflate(R.layout.fragment_vote_list, container, false);

        ListView listView = view.findViewById(R.id.vote_list_view);
        ArrayAdapter<Voting> adapter = new VoteAdapter(this.getActivity(), getVoting());
        listView.setAdapter(adapter);

        return view;
    }

    private List<Voting> getVoting() {
        List<Voting> list = new ArrayList<>();
        list.add(new Voting("Ход голосования по вопросу замены прошлогодней плитки на московскую", 70, 23, 53));
        list.add(new Voting("Ход голосования по вопросу замены окон", 30, 70, 53));

        return list;
    }
}