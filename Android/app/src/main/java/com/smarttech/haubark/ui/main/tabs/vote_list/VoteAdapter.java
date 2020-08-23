package com.smarttech.haubark.ui.main.tabs.vote_list;

import android.app.Activity;
import android.content.Intent;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.ProgressBar;
import android.widget.TextView;

import com.smarttech.haubark.R;
import com.smarttech.haubark.data.Voting;
import com.smarttech.haubark.ui.main.VoteActivity;

import java.util.List;


public class VoteAdapter extends ArrayAdapter<Voting> {

    private final List<Voting> list;
    private final Activity context;

    public VoteAdapter(Activity context, List<Voting> list) {
        super(context, R.layout.fragment_vote_list_item, list);
        this.context = context;
        this.list = list;
    }

    static class ViewHolder {
        protected TextView title;
        protected ProgressBar progressYes;
        protected ProgressBar progressNo;
        protected ProgressBar progressAbstained;
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {
        View view = null;
        if (convertView == null) {
            LayoutInflater inflator = context.getLayoutInflater();
            view = inflator.inflate(R.layout.fragment_vote_list_item, null);
            final ViewHolder viewHolder = new ViewHolder();

            // Setup view
            viewHolder.title = (TextView) view.findViewById(R.id.voting_title);
            viewHolder.progressYes = (ProgressBar) view.findViewById(R.id.progress_yes);
            viewHolder.progressNo = (ProgressBar) view.findViewById(R.id.progress_no);
            viewHolder.progressAbstained = (ProgressBar) view.findViewById(R.id.progress_abstained);

            view.findViewById(R.id.voting_container).setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    context.startActivity(new Intent(context, VoteActivity.class));
                }
            });

            view.setTag(viewHolder);
        } else {
            view = convertView;
        }

        ViewHolder holder = (ViewHolder) view.getTag();

        // Setup values
        holder.title.setText(list.get(position).getTitle());
        holder.progressYes.setProgress(list.get(position).getVoteYes());
        holder.progressNo.setProgress(list.get(position).getVoteNo());
        holder.progressAbstained.setProgress(list.get(position).getVoteAbstained());

        return view;
    }

}
